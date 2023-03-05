use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

/// 收益计算结果
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FundIncomeVO {
    pub date:Option<String>, // 日期
    pub net_worth:Option<Decimal>,// 净值
    pub rise_rate:Option<f64>,// 涨幅%
    pub rise:Option<Decimal>,// 涨幅￥
    pub trade_type:Option<String>,// 交易类型
    pub trade_share:Option<u64>,// 交易份额
    pub hold_share:Option<u64>,// 持有份额
    pub hold_value:Option<Decimal>,// 持有总市值
    pub cash_out:Option<Decimal>,// 已套现额
    pub cost:Option<Decimal>,// 总成本价
    pub earning_rate:Option<f64>,// 收益率
}











