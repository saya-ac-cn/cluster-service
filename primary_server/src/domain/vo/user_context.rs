use crate::error::Error;
use actix_web::HttpRequest;
use actix_http::header::HeaderValue;
use actix_web::web::to;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation, Algorithm};
use log::error;
use rustflake::Snowflake;
use serde::{Deserialize, Serialize};
use crate::service::CONTEXT;
use crate::util;

/// 用户上下文
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct UserContext {
    // 账号
    pub account: String,
    // 姓名
    pub name: String,
    // 组织
    pub organize: u64,
    // 登录ip
    pub ip: String,
    // 登录城市
    pub city: String
}

impl UserContext {
    /// extract token detail
    /// secret: your secret string
    pub async fn extract_token(token:&str) -> Result<UserContext, Error> {
        let verify = UserContext::verify(token).await;
        if verify.is_err() {
            log::error!("check access_token is fail! cause by:{}",verify.unwrap_err());
            return Err(Error::from(format!("access_token is invalid!")));
        }
        let user_cache= CONTEXT.redis_service.get_string(&format!("{:}:{:}", &util::USER_CACHE_PREFIX, token)).await;
        if user_cache.is_err(){
            log::error!("take user context fail!! cause by:{}",user_cache.unwrap_err());
            return Err(Error::from(format!("take user context fail!")));
        }
        let user_data: UserContext = serde_json::from_str(user_cache.unwrap().as_str()).unwrap();
        return Ok(user_data);
    }

    /// extract token detail
    /// secret: your secret string
    pub async fn extract_token_by_header(token:Option<&HeaderValue>) -> Result<UserContext, Error> {
        return match token {
            Some(token) => {
                let token:&str = token.to_str().unwrap_or("");
                UserContext::extract_token(token).await
            }
            _ => {
                log::error!("access_token is empty!");
                Err(Error::from(format!("access_token is empty!")))
            }
        }
    }

    /// extract token detail
    /// secret: your secret string
    pub async fn extract_user_by_header(token:Option<&HeaderValue>) -> Option<UserContext>{
        let extract_result = &UserContext::extract_token_by_header(token).await;
        if extract_result.is_err() {
            log::error!("在获取用户信息时，发生异常:{}",extract_result.clone().unwrap_err().to_string());
            return None;
        }
        let user_session = extract_result.clone().unwrap();
        return Some(user_session);
    }

    /// extract token detail
    /// secret: your secret string
    pub async fn extract_user_by_request(req: &HttpRequest) -> Option<UserContext>{
        let token = req.headers().get("access_token");
        UserContext::extract_user_by_header(token).await
    }

    /// create token
    /// secret: your secret string
    pub async fn create_token(account: &str) -> Result<String, Error> {
        // 生成用户token，在这里用account:雪花算法拼接，好处是，在判断用户是否已经登录时，直接通过account:判断
        let token = format!("{:}:{:}", account, &Snowflake::default().generate());
        return Ok(token);
    }

    /// verify token invalid
    /// secret: your secret string
    pub async fn verify(token: &str) -> Result<bool, Error> {
        return match CONTEXT.redis_service.exists(&format!("{:}:{:}", &util::USER_CACHE_PREFIX, token)).await{
            Ok(exists) => {
                match exists{
                    true => {
                        Ok(true)
                    }
                    false => {
                        log::error!("InvalidToken! token:{}",token);
                        return Err(Error::from("InvalidToken"));
                    }
                }
            }
            Err(err) => {
                log::error!("check redis user token cache data fail! token:{:?}",err);
                return Err(Error::from("InvalidToken other errors"));
            }
        };
    }
}
