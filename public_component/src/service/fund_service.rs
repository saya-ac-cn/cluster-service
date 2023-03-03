use std::collections::HashMap;
use std::fmt::format;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::ops::{Add, Mul};
use std::path::Path;
use chrono::{DateTime, FixedOffset, Local, NaiveDateTime};
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;
use serde_json::Value;
use crate::vo::fund_info_vo::FundInfoDTO;
use crate::vo::fund_net_worth_trend_vo::FundDataNetWorthTrendDTO;
use crate::dto::fund_setting_dto::FundSettingDTO;
use crate::service::FUND_GAINS;
use crate::util::Result;
use crate::util::Error;

pub struct FundService {}

impl FundService {

    /// 获取实时的基金详情
    pub async fn get_funds_info(&self, fund_code: &str) -> Result<FundInfoDTO> {
        let now = Local::now();
        let mut map = HashMap::new();
        // 加上时间戳，避免缓存
        map.insert("v",now.timestamp());
        let client = reqwest::Client::builder().build().unwrap();
        // https://fundgz.1234567.com.cn/js/007345.js?v=20200908175500
        let send_result = client.get(format!("https://fundgz.1234567.com.cn/js/{}.js", fund_code)).query(&map).send().await;
        if send_result.is_err(){
            return Err(Error::from("实时获取基金数据失败，请稍后再试..."));
        }
        let read_result = send_result.unwrap().text().await;
        if read_result.is_err() {
            return Err(Error::from("处理基金数据失败，请检查接口返回数据..."));
        }
        let jsonp:String = read_result.unwrap();
        if jsonp.is_empty() {
            return Err(Error::from("未查询到基金数据，请稍后再试..."));
        }
        if jsonp.eq("jsonpgz();") {
            return Err(Error::from("未查询到基金数据，请稍后再试..."));
        }
        //let jsonp:String = String::from("jsonpgz({\"fundcode\":\"007345\",\"name\":\"富国科技创新灵活配置混合\",\"jzrq\":\"2023-02-24\",\"dwjz\":\"1.4784\",\"gsz\":\"1.4730\",\"gszzl\":\"-0.36\",\"gztime\":\"2023-02-27 15:00\"});");
        let mut json:String = String::new();
        // 过滤掉jsonp
        if jsonp.starts_with("jsonpgz(") {
            json = String::from(jsonp.strip_prefix("jsonpgz(").unwrap())
        }
        if jsonp.ends_with(");") {
            json = String::from(json.strip_suffix(");").unwrap())
        }
        if json.is_empty() {
            // json是空的
            return Err(Error::from("未查询到基金数据，请稍后再试..."));
        }

        let json = serde_json::from_str(json.as_str());
        if json.is_err() {
            // json转换失败
            return Err(Error::from("未查询到基金数据，请稍后再试..."));
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
        //println!("{}",format!("基金代码:{},基金名称:{},净值日期:{},当日净值:{},估算净值:{},涨跌{}------",found.fundcode.unwrap(),found.name.unwrap(),found.jzrq.unwrap(),found.dwjz.unwrap(),found.gsz.unwrap(),found.gszzl.unwrap()));
        return Ok(found);;
    }

    /// 获取基金历史收益
    pub async fn get_fund_net_worth_trend(&self, fund_code: &str) -> Result<FundDataNetWorthTrendDTO>{
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
        //      return Err(Error::from("获取基金历史数据失败，请稍后再试..."));
        // }
        // let read_result = send_result.unwrap().text().await;
        // if read_result.is_err() {
        //    return Err(Error::from("处理基金历史数据失败，请稍后再试..."));
        // }
        // let jsonp:String = read_result.unwrap();
        // if jsonp.is_empty() {
        //    return Err(Error::from("未查询到基金历史数据，请稍后再试..."));
        // }
        // if jsonp.eq("jsonpgz();") {
        //    return Err(Error::from("未查询到基金历史数据，请稍后再试..."));
        // }
        let respones = fs::read_to_string(Path::new("./src/service/funds.txt"))?;


        // Data_netWorthTrend ;/*累计净值走势*/
        let start_bytes = respones.find("Data_netWorthTrend =").unwrap_or(0);
        let end_bytes = respones.find(";/*累计净值走势*/").unwrap_or(0);
        if start_bytes == 0 || end_bytes == 0 {
            return Err(Error::from("未能提取到累计净值走势数据，请稍后再试..."));
        }
        // 使用切片语法截取中间的部分
        let result = &respones[(start_bytes+"Data_netWorthTrend =".len())..end_bytes];

        // 过滤掉jsonp
        let json = serde_json::from_str(result);
        if json.is_err() {
            return Err(Error::from("累计净值走势数据转换失败，请稍后再试..."));
        }
        let value:Value = json.unwrap();
        let array_wrap = value.as_array();
        if array_wrap.is_none() {
            return Err(Error::from("累计净值走势数据转换失败，请稍后再试..."));
        }
        let array:&Vec<Value> = array_wrap.unwrap();
        let aa:Vec<FundDataNetWorthTrendDTO> = Vec::new();
        for item in array {
            let timestamp = item["x"].as_i64().unwrap();
            let naive = NaiveDateTime::from_timestamp_millis(timestamp).unwrap();
            let datetime: DateTime<FixedOffset> = DateTime::from_utc(naive, FixedOffset::east_opt(8*3600).unwrap());
            let date = datetime.format("%Y-%m-%d");
            // 今日净值
            let y:Decimal = Decimal::from_f64(item["y"].as_f64().unwrap()).unwrap();
            // 涨幅%
            let equity_return:Decimal = Decimal::from_f64(item["equityReturn"].as_f64().unwrap()).unwrap();
            // 涨幅￥ => t2 - (t2/(1+涨幅%))
            let gains:Decimal = y - (y/(equity_return/Decimal::from(100)+Decimal::from(1)));

            let _item = FundDataNetWorthTrendDTO{
                equityReturn: Some(equity_return),
                gains: Some(gains),
                unitMoney: Some(String::from(item["unitMoney"].as_str().unwrap())),
                x: Some(timestamp),
                date: Some(date.to_string()),
                y: Some(Decimal::from_f64(item["y"].as_f64().unwrap()).unwrap()),
            };
            let mut vec = FUND_GAINS.lock().unwrap();
            vec.push(_item);
        }
        return Err(Error::from("累计净值走势数据转换失败，请稍后再试..."));
    }

    /// 计算基金历史收益
    pub async fn calculate_income(&self, arg: &FundSettingDTO) -> Result<FundDataNetWorthTrendDTO>{
        let vec = FUND_GAINS.lock().unwrap();
        // 当前持有信息
        let mut hold_detail:HashMap<Decimal,u64> = HashMap::new();
        // 总套现额
        let mut cash_out:Decimal = Decimal::from(0);
        // 持有份额，用于校验在卖出时，是否充足
        let mut hold:u64 = 0;
        // 按百分比

        let rise:Decimal = arg.rise.clone().unwrap();
        let buy:i32 = arg.buy.clone().unwrap();
        let fall:Decimal = arg.fall.clone().unwrap();
        let sell:i32 = arg.sell.clone().unwrap();
        for item in vec.to_vec() {
            let net_worth:Decimal = item.y.clone().unwrap();
            let equity_return:Decimal = item.equityReturn.clone().unwrap();
            println!("{}",format!("\n-----{:?},净值:{:?},涨幅:{:?}%------",item.date,net_worth,equity_return));
            if item.equityReturn >= rise {
                // 上涨的趋势
                if buy > 0 {
                    // 给予买入
                    println!("买入{}",buy);
                    if hold_detail.contains_key(&net_worth) {
                        // 原持有份额
                        let tranche = hold_detail.get(&y).unwrap();
                        // 加仓
                        hold_detail.insert(y, (tranche+buy));
                    }else {
                        hold_detail.insert(y, buy as u64);
                    }
                    hold = hold + buy;
                }else {
                    // 给予卖出
                    // 检查是否充足
                    let _sell:i32 = buy.abs();
                    if hold < _sell {
                        println!("干啥勒，份额不足");
                        continue;
                    }
                    println!("卖出{}",_sell);
                    hold = hold - _sell;
                    // 给予卖出
                    // 添加到总套现中
                    cash_out = cash_out.add(net_worth.mul(Decimal::from(_sell)));
                }

                // 计算在以前买入 到现在的收益（暂时不考虑手续费）
                let (_cost,_sell) = compute_earnings(&mut hold_detail, &y);
                println!("{}",format!("->结算[{}]收益,持有份额:{},持有总市值:{},已套现额:{},总成本价:{},收益率{}%------",date,hold,(_sell-sell),sell,_cost,((_sell-_cost)/_cost*Decimal::from(100)).round_dp(5)));
                continue
            }
            if item.equityReturn < fall {
                // 下跌的趋势
                if sell > 0 {
                    // 给予卖出
                }else {
                    // 给予买入
                    let _buy:i32 = sell.abs();
                }





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
                continue;
            }
            let (_cost,_sell) = compute_earnings(&mut hold_detail, &y);
            println!("{}",format!("->结算[{}]收益,持有份额:{},持有总市值:{},已套现额:{},总成本价:{},收益率{}%------",date,hold,(_sell-sell),sell,_cost,((_sell-_cost)/_cost*Decimal::from(100)).round_dp(5)));
        }
        return Err(Error::from("累计净值走势数据转换失败，请稍后再试..."));
    }
}
