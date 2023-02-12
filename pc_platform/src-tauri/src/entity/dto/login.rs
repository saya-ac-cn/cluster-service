use serde::{Deserialize, Serialize};

/// 登陆
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LoginDTO {
    pub account: Option<String>,
    pub password: Option<String>,
    pub code: Option<String>
}
