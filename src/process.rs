use super::cli::day::Day;
use crate::role::login::Login;
use crate::role::site;
use crate::role::site::*;
use crate::role::state::State;
use crate::role::student::Student;
use crate::role::ts::Ts;
use crate::utils::def;
use crate::utils::json;
use crate::utils::*;
use anyhow::{anyhow, Result};
use chrono::{DateTime, Local};
use std::collections::HashMap;

/// # Query the information of a student.
pub fn query_by_name(day: Day, name: String) -> Result<Vec<Site>> {
    let mut body = HashMap::new();
    body.insert("byType", "devcls");
    body.insert("classkind", "8");
    body.insert("cld_name", "default");
    body.insert("act", "get_rsv_sta");
    let date = match day {
        Day::Today => time::get_date_today("%Y-%m-%d"),
        Day::Tomorrow => time::get_date_tomorrow("%Y-%m-%d"),
    };
    body.insert("date", date.as_str());

    let mut ret: Vec<Site> = Vec::new();

    for (room_name, floor) in def::ROOMS.iter() {
        let room_id = floor.room_id().to_string();
        let mut data = body.clone();
        data.insert("room_id", room_id.as_str());

        let resp =
            http::post(def::DEVICE_URL, def::HEADERMAP.clone(), data).unwrap_or_else(|err| {
                panic!(
                    "parse error when scan student in {} \n detail: {}",
                    room_name, err
                )
            });

        match json::get_name_info(resp, name.clone()) {
            Ok(info) => {
                ret.append(info.into_iter().collect::<Vec<Site>>().as_mut());
            }
            Err(err) => return Err(err),
        }
    }
    Ok(ret)
}

/// # Query the information of a site.
pub fn query_by_site(day: Day, site: String) -> Result<Site> {
    let dev_id = site_name_to_id(site.clone());
    match dev_id {
        Ok(dev_id) => {
            let mut body = HashMap::new();
            let dev_id_binding = dev_id.to_string();
            body.insert("dev_id", dev_id_binding.as_str());
            body.insert("act", "get_rsv_sta");
            let date = match day {
                Day::Today => time::get_date_today("%Y-%m-%d"),
                Day::Tomorrow => time::get_date_tomorrow("%Y-%m-%d"),
            };
            body.insert("date", date.as_str());

            let resp = http::post(def::DEVICE_URL, def::HEADERMAP.clone(), body)
                .expect("net error when querying site");

            json::get_site_info(resp)
        }
        Err(e) => Err(e),
    }
}

/// # handle actual login to the server.
fn handle_login() -> Result<Student> {
    let mut login = Login::new("".to_string(), "".to_string());
    login.read_from_file().expect("read student info failed");

    let mut body = HashMap::new();
    body.insert("act", "login");
    body.insert("id", login.username());
    body.insert("pwd", login.password());
    let resp = http::post(def::LOGIN_URL, def::HEADERMAP.clone(), body);
    match resp {
        Ok(resp) => json::get_login_info(resp),
        Err(e) => Err(e),
    }
}

/// # login to the server.
pub fn login(username: String, password: String) -> Result<Student> {
    let student = Login::new(username, password);
    student.save_to_file().expect("save student info failed");
    handle_login()
}

/// # query the user status.
pub fn status() -> Result<Vec<State>> {
    //login
    handle_login()?;

    let mut body = HashMap::new();
    body.insert("act", "get_History_resv");
    body.insert("strat", "90");
    body.insert("StatFlag", "New");

    let resp = http::post(def::CENTER_URL, def::HEADERMAP.clone(), body);
    match resp {
        Ok(resp) => json::get_state_info(resp),
        Err(e) => Err(e),
    }
}

/// # cancel the reservation.
pub fn cancel(id: String) -> Result<String> {
    //login
    handle_login()?;

    let mut body = HashMap::new();
    body.insert("act", "del_resv");
    body.insert("id", id.as_str());

    let resp = http::post(def::RESERVE_URL, def::HEADERMAP.clone(), body);
    match resp {
        Ok(resp) => json::get_cancel_info(resp),
        Err(e) => Err(e),
    }
}

fn handle_reserve(site: String, day: Day, start: String, end: String) -> Result<String> {
    let id = site_name_to_id(site)?;

    let mut body = HashMap::new();
    body.insert("act", "set_resv");
    let id_binding = id.to_string();
    body.insert("dev_id", id_binding.as_str());
    let day = match day {
        Day::Today => time::get_date_today("%Y-%m-%d"),
        Day::Tomorrow => time::get_date_tomorrow("%Y-%m-%d"),
    };
    let start_time = format!("{} {}", day, start);
    let end_time = format!("{} {}", day, end);
    body.insert("start", start_time.as_str());
    body.insert("end", end_time.as_str());

    let resp = http::post(def::RESERVE_URL, def::HEADERMAP.clone(), body);
    match resp {
        Ok(resp) => json::get_reserve_info(resp),
        Err(e) => Err(e),
    }
}

/// # reserve the site.
pub fn reserve(sites: Option<Vec<String>>, day: Day, start: String, end: String) -> Result<()> {
    //login
    handle_login()?;

    match sites {
        Some(sites) => {
            for site in sites {
                let resp = handle_reserve(site.clone(), day.clone(), start.clone(), end.clone());
                match resp {
                    Ok(resp) => {
                        println!("{}", format!("{}: {}\n", site, resp));
                        if resp.contains("成功") {
                            return Ok(());
                        }
                    }
                    Err(e) => return Err(e),
                }
            }
            Err(anyhow!("no site from put in can be reserved"))
        }
        None => {
            let mut cnt = 0;
            while cnt < 20 {
                let site = site::get_random_site_name()?;
                let resp = handle_reserve(site.clone(), day.clone(), start.clone(), end.clone());
                match resp {
                    Ok(resp) => {
                        println!("{}", format!("{}: {}\n", site, resp));
                        if resp.contains("成功") {
                            return Ok(());
                        }
                    }
                    Err(e) => return Err(e),
                }
                cnt += 1;
            }
            Err(anyhow!("time out for reserve random site"))
        }
    }
}

/// check out site
pub fn check_out(id: String) -> Result<String> {
    //login
    handle_login()?;

    let mut body = HashMap::new();
    body.insert("act", "resv_leave");
    body.insert("type", "2");
    body.insert("resv_id", id.as_str());

    let resp = http::post(def::RESERVE_URL, def::HEADERMAP.clone(), body);
    match resp {
        Ok(resp) => json::get_check_out_info(resp),
        Err(e) => Err(e),
    }
}