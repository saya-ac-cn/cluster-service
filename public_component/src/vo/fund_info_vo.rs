use serde::{Deserialize, Serialize};

/// 基金详情
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FundInfoVO {
    pub fundcode:Option<String>, //基金代码
    pub name:Option<String>,//基金名称
    pub jzrq:Option<String>,//净值日期
    pub dwjz:Option<String>,//当日净值
    pub gsz:Option<String>,//估算净值
    pub gszzl:Option<String>, //估算涨跌百分比 即-0.42%
    pub gztime:Option<String>,//估值时间
    pub start_date:Option<i64>,// 可以选择的开始时间
    pub end_date:Option<i64>// 可以选择的截止时间
}
