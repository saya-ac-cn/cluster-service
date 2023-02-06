use crate::domain::vo::user::SysUserVO;
use crate::domain::vo::SysRoleVO;
use serde::{Deserialize, Serialize};


///登录成功后的凭证数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SignInVO {
    pub user: Option<SysUserVO>,
    pub access_token: String,
}

impl ToString for SignInVO {
    fn to_string(&self) -> String {
        serde_json::json!(self).to_string()
    }
}
