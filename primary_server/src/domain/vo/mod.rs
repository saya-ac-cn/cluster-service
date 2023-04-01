pub mod user_context;
pub mod sign_in;
pub mod user;
pub mod log;
pub mod log_type;
pub mod plan;
pub mod plan_archive;
pub mod db_dump_log;
pub mod total_log;
pub mod total_table;
pub mod total_pre_6_month;

use crate::util::error::Error;
use actix_web::HttpResponse;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use crate::util;


/// The http interface returns the model structure, providing basic json data structures such as code, msg, and data
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RespVO<T> {
    pub code: Option<i32>,
    pub msg: Option<String>,
    pub data: Option<T>,
}

impl<T> RespVO<T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    pub fn from_result(arg: &Result<T, Error>) -> Self {
        if arg.is_ok() {
            Self {
                code: Some(util::SUCCESS_CODE),
                msg: None,
                data: arg.clone().ok(),
            }
        } else {
            let err:Error = arg.clone().err().unwrap();
            match err {
                Error::E(message, code) => {
                    Self {
                        code: Some(code),
                        msg: Some(message.to_string()),
                        data: None,
                    }
                }
            }
        }
    }


    pub fn resp_json(&self) -> HttpResponse {
        return HttpResponse::Ok()
            .insert_header(("Access-Control-Allow-Origin", "*"))
            .insert_header(("Cache-Control", "no-cache"))
            .insert_header(("Content-Type", "text/json;charset=UTF-8"))
            .body(self.to_string());
    }
}

impl<T> ToString for RespVO<T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
