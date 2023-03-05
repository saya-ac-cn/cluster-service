use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

/// 累计净值
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FundDataNetWorthTrendVO {
    pub equityReturn:Option<Decimal>, //净值回报率(就是当日涨跌幅)，保留到小数点后6位
    pub gains:Option<Decimal>,//涨幅（价格）
    pub unitMoney:Option<String>,//每份派送金
    pub x:Option<i64>,//净值日期(时间戳)
    pub date:Option<String>,//净值日期
    pub y:Option<Decimal>//单位净值
}
