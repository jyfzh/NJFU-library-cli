use crate::njfulib::floor::Floor;
use crate::utils::config;
use lazy_static::lazy_static;

pub const CONFIG_FILE: &str = ".njfulib.json";
pub const CACHE_FILE: &str = ".njfulib.cache";

pub const LONG_LINE_SEPARATOR: &str = "--------------------";

pub const SHORT_LINE_SEPARATOR: &str = "---------";

pub const BASE_URL: &str = "https://libseat.njfu.edu.cn/";
// 查询个人信息
pub const USER_INFO_URL: &str = "https://libic.njfu.edu.cn/ClientWeb/pro/ajax/login.aspx";
// 个人预约记录
pub const RESV_INFO_URL: &str = "https://libseat.njfu.edu.cn/ic-web/reserve/resvInfo";
// 取消
pub const CANCEL_URL: &str = "https://libseat.njfu.edu.cn/ic-web/reserve/delete";
// 预约
pub const RESERVE_URL: &str = "https://libseat.njfu.edu.cn/ic-web/reserve";
// 查询
pub const QUERY_URL: &str = "https://libseat.njfu.edu.cn/ic-web/reserve";
// 使用 resvId 查询用户信息
pub const SEARCHACCOUNT_URL: &str = "https://libseat.njfu.edu.cn/ic-web/reserve/getSignRec";

lazy_static! {
    pub static ref FLOORS: Vec<Floor> = {
        let mut floors: Vec<_> = Vec::new();
        floors.push(Floor::new(
            100455344,
            "2F-A".to_string(),
            100455361,
            100455800,
            440,
        ));
        floors.push(Floor::new(
            100455346,
            "2F-B".to_string(),
            100455802,
            100455897,
            96,
        ));
        floors.push(Floor::new(
            100455350,
            "3F-A".to_string(),
            100456256,
            100456658,
            403,
        ));
        floors.push(Floor::new(
            100455352,
            "3F-B".to_string(),
            100456660,
            100456791,
            132,
        ));
        floors.push(Floor::new(
            100455354,
            "3F-C".to_string(),
            100499567,
            100499728,
            162,
        ));
        floors.push(Floor::new(
            111488386,
            "3FA-".to_string(),
            111488493,
            111488512,
            20,
        ));
        floors.push(Floor::new(
            100455356,
            "4F-A".to_string(),
            100499729,
            100500156,
            428,
        ));
        floors.push(Floor::new(
            111488388,
            "4FA-".to_string(),
            111488513,
            111488536,
            24,
        ));
        floors.push(Floor::new(
            100455358,
            "5F-A".to_string(),
            100500173,
            100500532,
            360,
        ));
        floors.push(Floor::new(
            100455360,
            "6F-A".to_string(),
            100500602,
            100500949,
            348,
        ));
        floors
    };
    pub static ref CLIENT: reqwest::Client = {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
                reqwest::header::USER_AGENT,
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.114 Safari/537.36"
                    .parse()
                    .unwrap()
            );
        headers.insert(
            reqwest::header::CACHE_CONTROL,
            reqwest::header::HeaderValue::from_static("private"),
        );

        let config = config::load_config_from_file().unwrap();

        headers.insert(
            reqwest::header::COOKIE,
            format!("ic-cookie={}", config.cookie).parse().unwrap(),
        );

        reqwest::ClientBuilder::new()
            .cookie_store(true)
            .default_headers(headers)
            .build()
            .expect("Failed to build client")
    };
}
