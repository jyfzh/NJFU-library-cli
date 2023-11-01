#[allow(dead_code)]
mod def;
mod utils;

pub mod error;
pub mod njfulib;

use crate::error::ReserveError;
use crate::error::RespError;

use crate::njfulib::resp::Data;
use crate::njfulib::resp::Resp;
use crate::njfulib::site::*;
use crate::njfulib::user;
use crate::utils::config::{self, Config};
use crate::utils::handle::handle_reserve;
use crate::utils::handle::{self, handle_status};
use crate::utils::{filter::handle_filter, *};
use anyhow::{anyhow, Context, Result};
use log::{info, warn};

use indicatif::{ProgressBar, ProgressStyle};

use futures::future::join_all;
use std::collections::HashMap;
use std::time::Duration;

pub fn login(username: String, password: String, cookie: String) -> Result<Resp> {
    let config = Config::new(username, password, cookie, None);
    info!("save login config to {}", def::CONFIG_FILE);
    config::save_config_to_file(&config)?;
    Ok(Resp::new(0, "login success".to_owned(), None))
}

pub async fn query_by_name(day: u32, name: String, filter: Option<Vec<String>>) -> Result<Resp> {
    // ulimit
    #[cfg(target_os = "linux")]
    {
        use rlimit::{setrlimit, Resource};
        let soft = 16384;
        let hard = soft * 2;
        setrlimit(Resource::NOFILE, soft, hard).unwrap();
    }

    let tasks = filter::handle_filter(filter)?
        .into_iter()
        .map(|floor| -> String {
            format!(
                "{}?roomIds={}&resvDates={}&sysKind=8",
                def::QUERY_URL,
                floor.room_id.to_string(),
                time::get_date_with_offset("%Y%m%d", day as i32)
            )
        })
        .map(|url| def::CLIENT.get(url).send())
        .collect::<Vec<_>>();

    info!("query floor tasks");
    let tasks = join_all(tasks)
        .await
        .into_iter()
        .try_fold(vec![], |mut acc, resp| match resp {
            Ok(value) => {
                acc.push(value.json::<Resp>());
                Ok(acc)
            }
            Err(e) => Err(e),
        })?;

    info!("handle resp tasks");
    let tasks = join_all(tasks)
        .await
        .into_iter()
        .try_fold(vec![], |mut acc, resp| match resp {
            Ok(resp) => {
                acc.push(handle::get_name_info(resp, name.clone()));
                Ok(acc)
            }
            Err(e) => Err(e),
        })?;

    info!("query name info by resv_id in resp tasks");
    let mut message = String::new();
    let data = join_all(tasks)
        .await
        .into_iter()
        .try_fold(vec![], |mut acc, resp| match resp {
            Ok(resp) => {
                if resp.code == 0 {
                    message = resp.message;
                    acc.push(resp.data.unwrap()[0].clone());
                }
                Ok(acc)
            }
            Err(e) => match e.downcast_ref::<RespError>() {
                Some(RespError::Nodata) => {
                    warn!("{}", e);
                    Ok(acc)
                }
                _ => return Err(e),
            },
        })?;

    if data.len() > 0 {
        Ok(Resp::new(0, message, Some(data)))
    } else {
        Err(anyhow!(RespError::Nodata))
    }
}

pub async fn query_by_site(day: u32, site: String) -> Result<Resp> {
    let floor = site_name_to_floor(site.clone())?;
    let room_id = floor.room_id.to_string();
    let site_id = site_name_to_id(site.clone())?;
    let site_index = site_id - floor.dev_start + 1;
    let date = time::get_date_with_offset("%Y%m%d", day as i32);

    let url = format!(
        "{}?roomIds={}&resvDates={}&sysKind=8",
        def::QUERY_URL,
        room_id,
        date
    );
    info!("query floor tasks");
    let resp = def::CLIENT.get(url).send().await?.json::<Resp>().await?;
    handle::get_site_info(resp, site_index).await
}

pub async fn status(day: u32) -> Result<Resp> {
    let begin_date = time::get_date_with_offset("%Y-%m-%d", -(day as i32));
    let end_date = time::get_date_with_offset("%Y-%m-%d", 2);
    let url = format!(
        "{}?beginDate={}&endDate={}",
        def::RESV_INFO_URL,
        begin_date,
        end_date
    );
    info!("query status tasks");
    handle_status(def::CLIENT.get(url).send().await?.json::<Resp>().await?)
}

pub async fn cancel(uuid: String) -> Result<Resp> {
    let mut body = HashMap::new();
    body.insert("uuid", uuid.as_str());

    info!("cancel tasks");
    Ok(def::CLIENT
        .post(def::CANCEL_URL)
        .json(&body)
        .send()
        .await?
        .json::<Resp>()
        .await?)
}

pub async fn reserve(
    sites: Option<Vec<String>>,
    filter: Option<Vec<String>>,
    day: u32,
    start: String,
    end: String,
    retry: u32,
) -> Result<Resp> {
    let start = format!("{}:00", start);
    let end = format!("{}:00", end);

    let config = config::load_config_from_file()?;
    let user = user::search_user_info(&config).await?;

    let appacc_no = user.accno.parse::<u32>()?;
    let filter: Vec<u32> = handle_filter(filter)?
        .into_iter()
        .map(|x| x.room_id)
        .collect();

    let sites = match sites {
        Some(sites) => {
            let mut ret = Vec::new();
            for site in sites.iter() {
                ret.push(site_name_to_id(site.clone())?);
            }
            ret
        }
        None => {
            let mut ret = Vec::new();
            for _ in 0..=retry {
                ret.push(get_random_site_id(filter.clone())?);
            }
            ret
        }
    };

    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(100));
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.dim.bold} {prefix:.bold.dim} query floor: {wide_msg}",
        )
        .context("Failed to set progress bar style.")?
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ "),
    );

    for &site in sites.iter() {
        pb.set_prefix(format!("[{}]", site_id_to_name(site)?));
        let resp = site_reserve(site, appacc_no, day, start.clone(), end.clone()).await;
        match resp {
            Ok(ret) => {
                pb.finish_and_clear();
                return Ok(ret);
            }
            Err(e) => match e.downcast_ref::<RespError>().unwrap() {
                RespError::Reserve(err) => match err {
                    ReserveError::SiteAlreadReserved => {
                        pb.set_message(e.to_string());
                        continue;
                    }
                    ReserveError::Unknown(_) => {
                        pb.finish_and_clear();
                        return Err(e);
                    }
                },
                _ => {
                    pb.finish_and_clear();
                    return Err(e);
                }
            },
        }
    }
    pb.finish_and_clear();
    Err(anyhow!(RespError::Reserve(ReserveError::Unknown(
        "no site reserve success".to_owned()
    ))))
}

async fn site_reserve(
    site_id: u32,
    appacc_no: u32,
    day: u32,
    start: String,
    end: String,
) -> Result<Resp> {
    let date = time::get_date_with_offset("%Y-%m-%d", day as i32);
    let start_time = format!("{} {}", date, start);
    let end_time = format!("{} {}", date, end);

    let data = serde_json::json!({
        "sysKind": 8,
        "appAccNo": appacc_no,
        "resvMember": [ appacc_no ],
        "resvBeginTime": start_time,
        "resvEndTime": end_time,
        "resvDev": [ site_id ],
    });

    info!("reserve site_id: {}", site_id);

    let ret = def::CLIENT
        .post(def::RESERVE_URL)
        .json(&data)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    let code = ret["code"].as_u64().unwrap() as u32;
    let message = ret["message"].as_str().unwrap().to_owned();
    // NOTE: data is not a vec in response
    let data = match serde_json::from_value(ret["data"].clone()) {
        Ok(data) => Some(vec![Data::Status(data)]),
        Err(_) => None,
    };

    handle_reserve(Resp::new(code, message, data))
}
