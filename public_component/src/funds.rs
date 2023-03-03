use chrono::prelude::*;
use std::collections::HashMap;
use serde_json::Value;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::ops::{Add, Mul, Sub};
use std::path::Path;
use std::str::FromStr;
use std::time::Duration;
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;


/// 基金详情
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FundInfoDTO {
    pub fundcode:Option<String>, //基金代码
    pub name:Option<String>,//基金名称
    pub jzrq:Option<String>,//净值日期
    pub dwjz:Option<String>,//当日净值
    pub gsz:Option<String>,//估算净值
    pub gszzl:Option<String>, //估算涨跌百分比 即-0.42%
    pub gztime:Option<String>//估值时间
}

/// 累计净值
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FundDataNetWorthTrendDTO {
    pub equityReturn:Option<Decimal>, //净值回报率(就是当日涨跌幅)，保留到小数点后6位
    pub unitMoney:Option<String>,//每份派送金
    pub x:Option<i64>,//净值日期(时间戳)
    pub date:Option<String>,//净值日期
    pub y:Option<Decimal>//单位净值
}


pub async fn get_funds_info() -> Option<FundInfoDTO>{
    let now = Local::now();
    let mut map = HashMap::new();
    map.insert("v",now.timestamp());
    let client = reqwest::Client::builder().build().unwrap();
    // https://fundgz.1234567.com.cn/js/007345.js?v=20200908175500
    let send_result = client.get("https://fundgz.1234567.com.cn/js/001186.js").query(&map).send().await;
    if send_result.is_err(){
        println!("实时获取基金失败，请稍后再试...");
        return None;
    }
    let read_result = send_result.unwrap().text().await;
    if read_result.is_err() {
       println!("处理基金数据失败，请稍后再试");
        return None;
    }
    let jsonp:String = read_result.unwrap();
    if jsonp.is_empty() {
        println!("未查询到基金数据，请稍后再试");
        return None;
    }
    if jsonp.eq("jsonpgz();") {
        println!("未查询到基金数据，请稍后再试");
        return None;
    }
    //let jsonp:String = String::from("jsonpgz({\"fundcode\":\"007345\",\"name\":\"富国科技创新灵活配置混合\",\"jzrq\":\"2023-02-24\",\"dwjz\":\"1.4784\",\"gsz\":\"1.4730\",\"gszzl\":\"-0.36\",\"gztime\":\"2023-02-27 15:00\"});");
    let mut json:String = String::new();
    if jsonp.starts_with("jsonpgz(") {
        json = String::from(jsonp.strip_prefix("jsonpgz(").unwrap())
    }
    if jsonp.ends_with(");") {
        json = String::from(json.strip_suffix(");").unwrap())
    }
    if json.is_empty() {
        return None;
    }

    // 过滤掉jsonp
    let json = serde_json::from_str(json.as_str());
    if json.is_err() {
        return None;
    }
    let value:Value = json.unwrap();
    let found = FundInfoDTO{
        fundcode:Some(String::from(value["fundcode"].as_str().unwrap())),
        name:Some(String::from(value["name"].as_str().unwrap())),
        jzrq:Some(String::from(value["jzrq"].as_str().unwrap())),
        dwjz:Some(String::from(value["dwjz"].as_str().unwrap())),
        gsz:Some(String::from(value["gsz"].as_str().unwrap())),
        gszzl:Some(String::from(value["gszzl"].as_str().unwrap())),
        gztime:Some(String::from(value["gztime"].as_str().unwrap())),
    };
    println!("{}",format!("基金代码:{},基金名称:{},净值日期:{},当日净值:{},估算净值:{},涨跌{}------",found.fundcode.unwrap(),found.name.unwrap(),found.jzrq.unwrap(),found.dwjz.unwrap(),found.gsz.unwrap(),found.gszzl.unwrap()));
    return None;
}


pub async fn get_funds_list() -> Option<String>{
    // let mut map = HashMap::new();
    // map.insert("type","lsjz");
    // map.insert("code","007345");
    // map.insert("sdate","2020-09-01");
    // map.insert("edate","2020-09-28");
    // map.insert("page","1");
    // map.insert("per","30");
    // let client = reqwest::Client::builder().build().unwrap();
    // // https://fundf10.eastmoney.com/F10DataApi.aspx?type=lsjz&code=007345&page=1&per=20&sdate=2020-09-01&edate=2020-09-28
    // let send_result = client.get("https://fundf10.eastmoney.com/F10DataApi.aspx").query(&map).send().await;
    // if send_result.is_err(){
    //     // 获取基金历史数据失败，请稍后再试...
    //     return None;
    // }
    // let read_result = send_result.unwrap().text().await;
    // if read_result.is_err() {
    //     // 处理基金历史数据失败，请稍后再试
    //    return None;
    // }
    // let jsonp:String = read_result.unwrap();
    // if jsonp.is_empty() {
    //     // 未查询到基金历史数据，请稍后再试
    //     return None;
    // }
    // if jsonp.eq("jsonpgz();") {
    //     // 未查询到基金数据，请稍后再试
    //     return None
    // }
    let path = Path::new("C:\\Users\\Pandora\\Desktop\\funds.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                           why.description()),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut respones:String = String::new();
    match file.read_to_string(&mut respones) {
        Err(why) => panic!("couldn't read {}: {}", display,
                           why.description()),
        Ok(_) => ()//print!("{} contains:\n{}", display, respones),
    }
    // Data_netWorthTrend ;/*累计净值走势*/
    let start_bytes = respones.find("Data_netWorthTrend =").unwrap_or(0);
    let end_bytes = respones.find(";/*累计净值走势*/").unwrap_or(0);
    if start_bytes == 0 || end_bytes == 0 {
        println!("未能提取到累计净值走势数据，请稍后再试");
        return None;
    }
    // 使用切片语法截取中间的部分
    let result = &respones[(start_bytes+"Data_netWorthTrend =".len())..end_bytes];

    // 过滤掉jsonp
    let json = serde_json::from_str(result);
    if json.is_err() {
        println!("累计净值走势数据转换失败，请稍后再试");
        return None;
    }
    let value:Value = json.unwrap();
    let array_wrap = value.as_array();
    if array_wrap.is_none() {
        println!("累计净值走势数据转换失败，请稍后再试");
        return None;
    }
    let array:&Vec<Value> = array_wrap.unwrap();
    // 当前持有信息
    let mut hold_detail:HashMap<Decimal,u64> = HashMap::new();
    // 总套现额
    let mut sell:Decimal = Decimal::from(0);
    // 持有份额，用于校验在卖出时，是否充足
    let mut hold:u64 = 0;
    for item in array {
        let timestamp = item["x"].as_i64().unwrap();
        let naive = NaiveDateTime::from_timestamp_millis(timestamp).unwrap();
        let datetime: DateTime<FixedOffset> = DateTime::from_utc(naive, FixedOffset::east_opt(8*3600).unwrap());
        let date = datetime.format("%Y-%m-%d");
        // let _item = FundDataNetWorthTrendDTO{
        //     equityReturn: Some(Decimal::from_f64(item["equityReturn"].as_f64().unwrap()).unwrap()),
        //     unitMoney: Some(String::from(item["unitMoney"].as_str().unwrap())),
        //     x: Some(timestamp),
        //     date: Some(date.to_string()),
        //     y: Some(Decimal::from_f64(item["y"].as_f64().unwrap()).unwrap()),
        // };
        // 今日净值
        let y:Decimal = Decimal::from_f64(item["y"].as_f64().unwrap()).unwrap();
        // 涨幅
        let equity_return = Decimal::from_f64(item["equityReturn"].as_f64().unwrap()).unwrap();


        println!("{}",format!("\n-----{},净值:{},涨幅:{}%------",date,y,equity_return));
        if equity_return >= Decimal::from(1) {
            // 给予买入
            println!("买入1000");
            if hold_detail.contains_key(&y) {
                // 原持有份额
                let tranche = hold_detail.get(&y).unwrap();
                // 加仓
                hold_detail.insert(y, (tranche+1000));
            }else {
                hold_detail.insert(y, 1000);
            }
            hold = hold + 1000;
            // 计算在以前买入 到现在的收益（暂时不考虑手续费）
            let (_cost,_sell) = compute_earnings(&mut hold_detail, &y);
            println!("{}",format!("->结算[{}]收益,持有份额:{},持有总市值:{},已套现额:{},总成本价:{},收益率{}%------",date,hold,(_sell-sell),sell,_cost,((_sell-_cost)/_cost*Decimal::from(100)).round_dp(5)));
            //println!("{}",format!("->结算[{}]收益,持有份额:{},持有总市值:{},已套现额:{},总成本价:{}------",date,hold,(_sell-sell),sell,_cost));
            continue
        }
        if equity_return <= Decimal::from(-1) {
            if hold < 1000 {
                println!("干啥勒，份额不足");
                continue;
            }
            hold = hold - 1000;
            // 给予卖出
            println!("卖出1000");
            // 添加到总套现中
            sell = sell.add(y.mul(Decimal::from(1000)));
            // 套现后 触发一次持有收益的计算
            let (_cost,_sell) = compute_earnings(&mut hold_detail, &y);
            println!("{}",format!("->结算[{}]收益,持有份额:{},持有总市值:{},已套现额:{},总成本价:{},收益率{}%------",date,hold,(_sell-sell),sell,_cost,((_sell-_cost)/_cost*Decimal::from(100)).round_dp(5)));
            //println!("{}",format!("->结算[{}]收益,持有份额:{},持有总市值:{},已套现额:{},总成本价:{}------",date,hold,(_sell-sell),sell,_cost));
            continue;
        }
        let (_cost,_sell) = compute_earnings(&mut hold_detail, &y);
        println!("{}",format!("->结算[{}]收益,持有份额:{},持有总市值:{},已套现额:{},总成本价:{},收益率{}%------",date,hold,(_sell-sell),sell,_cost,((_sell-_cost)/_cost*Decimal::from(100)).round_dp(5)));
    }

    return Some(String::from("89u898"));
}

pub fn compute_earnings(map: &mut HashMap<Decimal,u64>,net_worth:&Decimal) -> (Decimal,Decimal){
    // 持有总成本价
    let mut cost:Decimal = Decimal::from(0);
    // 持有总市值
    let mut sell:Decimal = Decimal::from(0);
    for (key, value) in map.iter() {
        // 持有份额
        let number: Decimal = Decimal::from(*value);
        cost = cost.add(number.mul(key));
        sell = sell.add(number.mul(net_worth));
    }
    (cost,sell)
}

#[cfg(test)]
mod test {
    use regex::Regex;
    use public_component::service::{FUND_GAINS, FundService};
    use crate::funds::{get_funds_info, get_funds_list};

    #[tokio::test]
    async fn main(){
        //get_funds_info().await;
        //get_funds_list().await;

        let service = FundService {};
        // let funds_info = service.get_funds_info("007345").await;
        // if funds_info.is_ok() {
        //     let found = funds_info.unwrap();
        //     println!("{}",format!("基金代码:{},基金名称:{},净值日期:{},当日净值:{},估算净值:{},涨跌{}",found.fundcode.unwrap(),found.name.unwrap(),found.jzrq.unwrap(),found.dwjz.unwrap(),found.gsz.unwrap(),found.gszzl.unwrap()));
        // }
        service.get_fund_net_worth_trend("007345").await;
        let vec = FUND_GAINS.lock().unwrap();
        for item in vec.to_vec() {
            println!("{:?}",item)
        }

        //println!("funds_info:{}",get_funds_info());
    }


    fn test2() {
        let s = String::from("hello [world]");
        // 定义正则表达式
        let re = Regex::new(r"\[(\w+)\]").unwrap();
        // 使用 captures() 方法提取括号中的内容
        if let Some(captures) = re.captures(&s) {
            let content = captures.get(1).unwrap().as_str();
            println!("{}", content); // 输出 "world"
        }
    }

    // #[test]
    // fn get_funds(){
    //     let mut map = HashMap::new();
    //     map.insert("key",&CONTEXT.config.amap_key);
    //     map.insert("ip",ip);
    //     let client = reqwest::Client::builder().build().unwrap();
    //     // https://fundf10.eastmoney.com/F10DataApi.aspx?type=lsjz&code=007345&page=1&per=20&sdate=2020-09-01&edate=2020-09-18
    //     let send_result = client.get( &CONTEXT.config.amap_url).query(&map).send().await;
    //     if send_result.is_err(){
    //         return String::from("定位失败");
    //     }
    //     let read_result = send_result.unwrap().text().await;
    //     if read_result.is_err() {
    //         return String::from("定位失败");
    //     }
    //     let json_str = read_result.unwrap();
    //     let json = serde_json::from_str(json_str.as_str());
    //     if json.is_err() {
    //         return String::from("定位失败");
    //     }
    //     let location:Value = json.unwrap();
    //     return format!("{}{}",location["province"].as_str().unwrap(),location["city"].as_str().unwrap());
    // }

}
