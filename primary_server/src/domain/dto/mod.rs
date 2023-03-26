pub mod user;
pub mod sign_in;
pub mod db_dump_log;
pub mod log;
pub mod log_type;
pub mod plan;
pub mod plan_archive;
pub mod page;


use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EmptyDTO {}

/// IdDTO
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IdDTO {
    pub id: Option<String>,
}
