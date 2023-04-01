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
            .route("/backend/logout", web::post().to(system_controller::logout),)
            // 映射静态资源目录
            //.service(fs::Files::new("/warehouse", &CONTEXT.config.data_dir))
            .service(
                web::scope("/backend/system")
                    .service(system_controller::myself)
                    .service(system_controller::user_add)
                    .service(system_controller::user_update)
                    .service(system_controller::user_detail)
                    .service(system_controller::user_remove)
                    .service(system_controller::user_page)
                    .service(system_controller::own_organize_user)
                    // .service(system_controller::user_upload_logo)
                    .service(system_controller::user_update_password)
                    .service(system_controller::log_page)
                    .service(system_controller::log_excel)
                    .service(system_controller::log_type)
                    // .service(system_controller::compute_pre6_logs)
                    // .service(system_controller::compute_object_rows)
                    .service(system_controller::add_notes)
                    .service(system_controller::edit_plan)
                    .service(system_controller::delete_plan)
                    .service(system_controller::plan_page)
                    .service(system_controller::finish_plan)
                    .service(system_controller::plan_archive_page)
                    .service(system_controller::edit_archive_plan)
                    .service(system_controller::delete_archive_plan)
                    .service(system_controller::db_dump_log_page)
            )
    })
    .bind(&CONTEXT.config.server_url)?
    .run()
    .await
}


