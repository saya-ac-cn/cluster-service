mod sys_user_service;
mod redis_service;

use crate::service::sys_user_service::SysUserService;
use crate::service::redis_service::RedisService;

pub use crate::config::config::ApplicationConfig;
use once_cell::sync::Lazy;
use rbatis::rbatis::Rbatis;


/// CONTEXT is all of the service struct
pub static CONTEXT: Lazy<ServiceContext> = Lazy::new(|| ServiceContext::default());

#[macro_export]
macro_rules! pool {
    () => {
        &mut $crate::service::CONTEXT.rb.clone()
    };
}

pub struct ServiceContext {
    pub config: ApplicationConfig,
    pub rb: Rbatis,
    pub sys_user_service: SysUserService,
    pub redis_service: RedisService,
}

impl ServiceContext {
    /// init database pool
    pub async fn init_pool(&self) {
        log::info!(
            "[primary_server] rbatis pool init ({})...",
            self.config.database_url
        );
        let driver = rbdc_mysql::driver::MysqlDriver {};
        let driver_name = format!("{:?}", driver);
        self.rb
            .init(driver, &self.config.database_url)
            .expect("[primary_server] rbatis pool init fail!");
        self.rb.acquire().await.expect(&format!(
            "rbatis connect database(driver={},url={}) fail",
            driver_name, self.config.database_url
        ));
        log::info!(
            "[primary_server] rbatis pool init success! pool state = {:?}",
            self.rb.get_pool().expect("pool not init!").status()
        );
        log::info!(
            " - Local:   http://{}",
            self.config.server_url.replace("0.0.0.0", "127.0.0.1")
        );
    }
}

impl Default for ServiceContext {
    fn default() -> Self {
        let config = ApplicationConfig::default();
        ServiceContext {
            rb: crate::domain::init_rbatis(&config),
            sys_user_service: SysUserService {},
            redis_service: RedisService::new(&config.redis_url),
            config,
        }
    }
}
