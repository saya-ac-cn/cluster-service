use primary_server::service::CONTEXT;
use actix_web::{web, App, HttpResponse, HttpServer, Responder, HttpRequest};
use primary_server::controller::system_controller;
use primary_server::middleware::auth_actix::Auth;
use primary_server::util::scheduler::Scheduler;

/// use tokio,because Rbatis specifies the runtime-tokio
#[tokio::main]
async fn main() -> std::io::Result<()> {
    // log
    primary_server::config::log::init_log();
    // database
    CONTEXT.init_pool().await;
    // scheduler
    //actix_web::rt::spawn(Scheduler::start()).await;
    Scheduler::start().await;
    // router
    HttpServer::new(|| {
        App::new()
            .wrap(Auth {})
            // 登录登出接口单独处理（因为都不在已有的分组中）
            .route("/backend/login", web::post().to(system_controller::login),)
            //.route("/backend/logout", web::post().to(system_controller::logout),)
            // 映射静态资源目录
            //.service(fs::Files::new("/warehouse", &CONTEXT.config.data_dir))
    })
    .bind(&CONTEXT.config.server_url)?
    .run()
    .await
}


