use financial_server::service::CONTEXT;
use actix_web::{web, App, HttpResponse, HttpServer, Responder, HttpRequest};
use financial_server::controller::sys_user_controller;
use financial_server::middleware::auth_actix::Auth;

/// use tokio,because Rbatis specifies the runtime-tokio
#[tokio::main]
async fn main() -> std::io::Result<()> {
    //log
    financial_server::config::log::init_log();
    //database
    CONTEXT.init_pool().await;
    //router
    HttpServer::new(|| {
        App::new()
            .wrap(Auth {})
            .route("/login", web::post().to(sys_user_controller::login))
    })
    .bind(&CONTEXT.config.server_url)?
    .run()
    .await
}
