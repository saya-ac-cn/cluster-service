mod cache_service;
mod mem_service;
mod redis_service;
mod sys_dict_service;
mod sys_user_service;

pub use crate::config::config::ApplicationConfig;
pub use cache_service::*;
pub use mem_service::*;
use once_cell::sync::Lazy;
use rbatis::rbatis::Rbatis;
use rbdc_mysql::driver::MysqlDriver;
pub use redis_service::*;
pub use sys_dict_service::*;
pub use sys_user_service::*;

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
    pub cache_service: CacheService,
    pub sys_user_service: SysUserService,
    pub sys_dict_service: SysDictService
}

impl ServiceContext {
    /// init database pool
    pub async fn init_pool(&self) {
        //连接数据库
        println!(
            "[abs_admin] rbatis pool init ({})...",
            self.config.database_url
        );
        self.rb
            .init(MysqlDriver {}, &self.config.database_url)
            .expect("[abs_admin] rbatis pool init fail!");
        log::info!(
            "[abs_admin] rbatis pool init success! pool state = {:?}",
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
            cache_service: CacheService::new(&config).unwrap(),
            sys_user_service: SysUserService {},
            sys_dict_service: SysDictService {},
            config,
        }
    }
}
