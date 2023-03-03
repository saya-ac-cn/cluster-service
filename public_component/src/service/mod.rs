pub mod fund_service;

use std::sync::Mutex;
use lazy_static::lazy_static;
pub use fund_service::*;
use crate::vo::fund_net_worth_trend_vo::FundDataNetWorthTrendDTO;



lazy_static! {
    /// CONTEXT is all of the service struct
    pub static ref CONTEXT: ServiceContext = ServiceContext::default();
    /// 基金收益
    pub static ref FUND_GAINS: Mutex<Vec<FundDataNetWorthTrendDTO>> = Mutex::new(Vec::new());
}


pub struct ServiceContext {
    pub fund_service: FundService
}


impl Default for ServiceContext {
    fn default() -> Self {
        ServiceContext {
            fund_service: FundService {}
        }
    }
}
