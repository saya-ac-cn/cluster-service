use business_server::service::CONTEXT;
use actix_web::{web, App, HttpResponse, HttpServer, Responder, HttpRequest};
use business_server::controller::sys_user_controller;
use business_server::middleware::auth_actix::Auth;

/// use tokio,because Rbatis specifies the runtime-tokio
#[tokio::main]
async fn main() -> std::io::Result<()> {
    //log
    business_server::config::log::init_log();
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


