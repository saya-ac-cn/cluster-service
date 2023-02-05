use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SignInDTO {
    // 账号
    pub account: String,
    // 密码
    pub password: String,
    // 校验方式
    pub check_method: String,
    //验证码，可用是短信验证码，图片验证码,二维码验证码...
    pub code: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CatpchaDTO {
    pub account: Option<String>,
}
