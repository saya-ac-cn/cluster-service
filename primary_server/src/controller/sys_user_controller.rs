use actix_web::{web, HttpRequest, Responder,HttpResponse};

use serde::{Deserialize, Serialize};
use crate::domain::dto::SignInDTO;
use crate::domain::vo::RespVO;
use crate::service::CONTEXT;

/// 用户登录
pub async fn login(req: HttpRequest,arg: web::Json<SignInDTO>) -> impl Responder {
    log::info!("login:{:?}", arg.0);
    let vo = CONTEXT.sys_user_service.login(&req,&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}
