use crate::domain::table::{LoginCheckEnum, SysUser};
use crate::domain::vo::SysRoleVO;
use rbatis::rbdc::datetime::FastDateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SysUserVO {
    pub id: Option<String>,
    pub account: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
    pub login_check: Option<LoginCheckEnum>,
    pub state: Option<i32>,
    pub del: Option<i32>,
    pub create_date: Option<FastDateTime>,

    pub role: Option<SysRoleVO>,
}

impl From<SysUser> for SysUserVO {
    fn from(arg: SysUser) -> Self {
        Self {
            id: arg.id,
            account: arg.account,
            //屏蔽密码
            password: None,
            name: arg.name,
            login_check: arg.login_check,
            state: arg.state,
            del: arg.del,
            create_date: arg.create_date,
            role: None,
        }
    }
}
