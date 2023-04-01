mod system_service;
mod redis_service;
use crate::service::redis_service::RedisService;

pub use crate::config::config::ApplicationConfig;
use crate::util::scheduler::Scheduler;
use rbatis::rbatis::Rbatis;
use std::sync::Mutex;
use lazy_static::lazy_static;
use crate::service::system_service::SystemService;

// 第一种初始化方法
// /// CONTEXT is all of the service struct
// pub static CONTEXT: Lazy<ServiceContext> = Lazy::new(|| ServiceContext::default());


// 在lazy_static! { //your code} 中的代码并不会在编译时初始化静态量，它会在首次调用时，执行代码，来初始化。也就是所谓的延迟计算。
lazy_static! {
    // CONTEXT is all of the service struct
    pub static ref CONTEXT: ServiceContext = ServiceContext::default();
    // SCHEDULER is only SCHEDULER VARIABLE
    pub static ref SCHEDULER: Mutex<Scheduler> = Mutex::new(Scheduler::default());
}


#[macro_export]
macro_rules! pool {
    () => {
        &mut $crate::service::CONTEXT.rb.clone()
    };
}

pub struct ServiceContext {
    pub config: ApplicationConfig,
    pub rb: Rbatis,
    pub system_service: SystemService,
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
            system_service: SystemService {},
            redis_service: RedisService::new(&config.redis_url),
            config,
        }
    }
}
