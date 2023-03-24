use crate::error::Error;
use crate::error::Result;
use crate::service::CONTEXT;
use rbatis::rbdc::types::datetime::DateTime;
use crate::domain::dto::{SignInDTO};
use crate::pool;

use std::time::Duration;
use actix_web::HttpRequest;
use crate::domain::table::User;
use crate::domain::vo::{JWTToken, SignInVO};
use crate::util::PasswordEncoder;


///Background User Service
pub struct SysUserService {}

impl SysUserService {

    /// 登录
    pub async fn login(&self, req: &HttpRequest, arg: &SignInDTO) -> Result<SignInVO> {
        //CONTEXT.redis_service.set_string("user","saya").await;
        let aa = CONTEXT.redis_service.exists("user1").await;
        println!("resulr{}",aa.unwrap());
        let mut sign_vo = SignInVO {
            user: None,
            access_token: String::new(),
            plan: None,
            log: None,
        };
        // 临时注释权限
        //提前查找所有权限，避免在各个函数方法中重复查找
        // let all_res = CONTEXT.sys_res_service.finds_all_map().await?;
        // sign_vo.permissions = self.load_level_permission(&user_id, &all_res).await?;
        let jwt_token = JWTToken {
            account: "189".to_string(),
            name: "1".to_string(),
            organize: 1,
            ip:String::from("127.0.0.1"),
            city:String::from("局域网"),
            exp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as usize,
        };
        // 构造token
        sign_vo.access_token = jwt_token.create_token(&CONTEXT.config.jwt_secret)?;
        return Ok(sign_vo);
        // 根据账号查询用户
        // let user: Option<User> = User::select_by_column(pool!(), "account", &arg.account).await?.into_iter().next();
        // let user = user.ok_or_else(|| Error::from(format!("账号:{} 不存在!", arg.account.clone().unwrap())))?;
        // // 检查是否被禁用
        // if user.state.eq(&Some(0)) {
        //     return Err(Error::from("账户被禁用!"));
        // }
        // // check pwd
        // if !PasswordEncoder::verify(
        //     user.password
        //         .as_ref()
        //         .ok_or_else(|| Error::from(("错误的用户数据，密码为空!")))?,
        //     &arg.password.clone().unwrap(),
        // ){
        //     return Err(Error::from("密码不正确!"));
        // }
        // // 封装前端所需要的资源
        // let sign_in_vo = self.get_user_info(&user).await?;
        // return Ok(sign_in_vo);
    }

    pub async fn get_user_info(&self, user: &User) -> Result<SignInVO> {
        //去除密码，增加安全性
        let mut user = user.clone();
        user.password = None;
        let mut sign_vo = SignInVO {
            user: Some(user.clone().into()),
            access_token: String::new(),
            plan: None,
            log: None,
        };
        let jwt_token = JWTToken {
            account: "189".to_string(),
            name: "1".to_string(),
            organize: 1,
            ip:String::from("127.0.0.1"),
            city:String::from("局域网"),
            exp: 0
        };
        sign_vo.access_token = jwt_token.create_token(&CONTEXT.config.jwt_secret)?;
        return Ok(sign_vo);
    }
}
