use std::collections::HashMap;
use std::fmt::format;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::ops::{Add, Div, Mul};
use std::path::Path;
use chrono::{DateTime, FixedOffset, Local, NaiveDateTime};
use rust_decimal::Decimal;
use rust_decimal::prelude::{FromPrimitive, ToPrimitive, Zero};
use serde_json::Value;
use crate::vo::fund_info_vo::FundInfoVO;
use crate::vo::fund_net_worth_trend_vo::FundDataNetWorthTrendVO;
use crate::dto::fund_setting_dto::FundSettingDTO;
use crate::service::FUND_GAINS;
use crate::util::Result;
use crate::util::Error;
use crate::vo::fund_income_vo::FundIncomeVO;
extern crate simple_excel_writer as excel;
use excel::*;

pub struct FundService {}

impl FundService {
    /// 获取实时的基金详情
    pub async fn get_funds_info(&self, fund_code: &str) -> Result<FundInfoVO> {
        let now = Local::now();
        let mut map = HashMap::new();
        // 加上时间戳，避免缓存
        map.insert("v", now.timestamp());
        let client = reqwest::Client::builder().build().unwrap();
        // https://fundgz.1234567.com.cn/js/007345.js?v=20200908175500
        let send_result = client.get(format!("https://fundgz.1234567.com.cn/js/{}.js", fund_code)).query(&map).send().await;
        if send_result.is_err() {
            return Err(Error::from("实时获取基金数据失败，请稍后再试..."));
        }
        let read_result = send_result.unwrap().text().await;
        if read_result.is_err() {
            return Err(Error::from("处理基金数据失败，请检查接口返回数据..."));
        }
        let jsonp: String = read_result.unwrap();
        if jsonp.is_empty() {
            return Err(Error::from("未查询到基金数据，请稍后再试..."));
        }
        if jsonp.eq("jsonpgz();") {
            return Err(Error::from("未查询到基金数据，请稍后再试..."));
        }
        //let jsonp:String = String::from("jsonpgz({\"fundcode\":\"007345\",\"name\":\"富国科技创新灵活配置混合\",\"jzrq\":\"2023-02-24\",\"dwjz\":\"1.4784\",\"gsz\":\"1.4730\",\"gszzl\":\"-0.36\",\"gztime\":\"2023-02-27 15:00\"});");
        let mut json: String = String::new();
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
        let value: Value = json.unwrap();
        let found = FundInfoVO {
            fundcode: Some(String::from(value["fundcode"].as_str().unwrap())),
            name: Some(String::from(value["name"].as_str().unwrap())),
            jzrq: Some(String::from(value["jzrq"].as_str().unwrap())),
            dwjz: Some(String::from(value["dwjz"].as_str().unwrap())),
            gsz: Some(String::from(value["gsz"].as_str().unwrap())),
            gszzl: Some(String::from(value["gszzl"].as_str().unwrap())),
            gztime: Some(String::from(value["gztime"].as_str().unwrap())),
            start_date:None,
            end_date:None
        };
        //println!("{}",format!("基金代码:{},基金名称:{},净值日期:{},当日净值:{},估算净值:{},涨跌{}------",found.fundcode.unwrap(),found.name.unwrap(),found.jzrq.unwrap(),found.dwjz.unwrap(),found.gsz.unwrap(),found.gszzl.unwrap()));
        return Ok(found);
    }

    /// 获取基金历史收益
    pub async fn get_fund_net_worth_trend(&self, fund_code: &str) -> Result<FundDataNetWorthTrendVO> {
        let now = Local::now();
        let mut map = HashMap::new();
        // 加上时间戳，避免缓存
        map.insert("v", now.timestamp());
        let client = reqwest::Client::builder().build().unwrap();
        let send_result = client.get(format!("http://fund.eastmoney.com/pingzhongdata/{}.js?v=20160518155842",fund_code)).query(&map).send().await;
        if send_result.is_err(){
             return Err(Error::from("获取基金历史数据失败，请稍后再试..."));
        }
        let read_result = send_result.unwrap().text().await;
        if read_result.is_err() {
           return Err(Error::from("处理基金历史数据失败，请稍后再试..."));
        }
        let respones:String = read_result.unwrap();
        if respones.is_empty() {
           return Err(Error::from("未查询到基金历史数据，请稍后再试..."));
        }
        if respones.eq("jsonpgz();") {
           return Err(Error::from("未查询到基金历史数据，请稍后再试..."));
        }
        // let respones = fs::read_to_string(Path::new("./src/service/funds.txt"))?;

        // Data_netWorthTrend ;/*累计净值走势*/
        let start_bytes = respones.find("Data_netWorthTrend =").unwrap_or(0);
        let end_bytes = respones.find(";/*累计净值走势*/").unwrap_or(0);
        if start_bytes == 0 || end_bytes == 0 {
            return Err(Error::from("未能提取到累计净值走势数据，请稍后再试..."));
        }
        // 使用切片语法截取中间的部分
        let result = &respones[(start_bytes + "Data_netWorthTrend =".len())..end_bytes];

        // 过滤掉jsonp
        let json = serde_json::from_str(result);
        if json.is_err() {
            return Err(Error::from("累计净值走势数据转换失败，请稍后再试..."));
        }
        let value: Value = json.unwrap();
        let array_wrap = value.as_array();
        if array_wrap.is_none() {
            return Err(Error::from("累计净值走势数据转换失败，请稍后再试..."));
        }
        let array: &Vec<Value> = array_wrap.unwrap();
        let aa: Vec<FundDataNetWorthTrendVO> = Vec::new();
        // 在查询基金后，将上次的计算结果清空
        let mut vec = FUND_GAINS.lock().unwrap();
        vec.clear();
        for item in array {
            let timestamp = item["x"].as_i64().unwrap();
            let naive = NaiveDateTime::from_timestamp_millis(timestamp).unwrap();
            let datetime: DateTime<FixedOffset> = DateTime::from_utc(naive, FixedOffset::east_opt(8 * 3600).unwrap());
            let date = datetime.format("%Y-%m-%d");
            // 今日净值
            let y: Decimal = Decimal::from_f64(item["y"].as_f64().unwrap()).unwrap();
            // 涨幅%
            let equity_return: Decimal = Decimal::from_f64(item["equityReturn"].as_f64().unwrap()).unwrap();
            // 涨幅￥ => t2 - (t2/(1+涨幅%))
            let gains: Decimal = y - (y / (equity_return / Decimal::from(100) + Decimal::from(1)));

            let _item = FundDataNetWorthTrendVO {
                equityReturn: Some(equity_return),
                gains: Some(gains.round_dp(5)),
                unitMoney: Some(String::from(item["unitMoney"].as_str().unwrap())),
                x: Some(timestamp),
                date: Some(date.to_string()),
                y: Some(Decimal::from_f64(item["y"].as_f64().unwrap()).unwrap()),
            };
            //let mut vec = FUND_GAINS.lock().unwrap();
            vec.push(_item);
        }
        return Err(Error::from("累计净值走势数据转换失败，请稍后再试..."));
    }

    /// 计算基金历史收益
    pub fn calculate_income(&self, arg: &FundSettingDTO) -> Result<Vec<FundIncomeVO>> {
        if arg.flag.clone().unwrap() {
            self.calculate_income_by_percentage(arg)
        } else {
            self.calculate_income_by_price(arg)
        }
    }

    /// 按照百分比
    pub fn calculate_income_by_percentage(&self,arg: &FundSettingDTO) -> Result<Vec<FundIncomeVO>> {
        let mut result:Vec<FundIncomeVO> = Vec::new();
        // 当前持有信息
        let mut hold_detail: HashMap<Decimal, u64> = HashMap::new();
        // 总套现额
        let mut cash_out: Decimal = Decimal::from(0);
        // 持有份额（买入、卖出需要更新），用于校验在卖出时，判断是否充足，以及时刻计算总市值（持有份额*当前单位市值）
        let mut hold: u64 = 0;
        // 本次实际实际交易份额（在持有不足的情况下，卖出只能全部卖出）
        let mut real_trade_share: u64 = 0;

        let start_date:i64 = arg.start_date.clone().unwrap();
        let end_date:i64 = arg.end_date.clone().unwrap();

        // 参数载入
        let vec = FUND_GAINS.lock().unwrap();
        let rise: Decimal = arg.rise.clone().unwrap();
        let buy: i32 = arg.buy.clone().unwrap();
        let fall: Decimal = arg.fall.clone().unwrap();
        let sell: i32 = arg.sell.clone().unwrap();
        // 初始持有信息

        let init_share:u64 = arg.init_share.clone().unwrap();
        let init_net_worth: Decimal = arg.init_net_worth.clone().unwrap();
        hold = self.buy_funds(init_share, &init_net_worth, &mut hold_detail, hold);

        for item in vec.to_vec() {
            let _current:i64 = item.x.clone().unwrap();
            if _current < start_date || _current > end_date {
                // 不在时间范围内，跳过不予处理
                continue;
            }

            // 当日涨幅 净值
            let net_worth: Decimal = item.y.clone().unwrap();
            let equity_return: Decimal = item.equityReturn.clone().unwrap();
            let gains: Decimal = item.gains.clone().unwrap();
            //println!("{}", format!("\n-----{:?},净值:{:?},涨幅:{:?}%------", item.date, net_worth, equity_return));
            if equity_return >= rise {
                // 上涨的趋势 正数为卖出，负数为买入
                let unit: u64 = self.compute_units(equity_return.clone(), rise.clone(), sell.abs() as u32);
                // 本次交易类型
                let mut trade_type:String = String::new();
                if sell > 0 {
                    // 给予卖出
                    (hold, cash_out,real_trade_share) = self.sell_funds(unit, &net_worth, cash_out.clone(), hold);
                    trade_type = if 0 == real_trade_share {String::from("-")}else{String::from("赎回")};
                } else {
                    // 给予买入
                    real_trade_share = unit;
                    hold = self.buy_funds(unit, &net_worth, &mut hold_detail, hold);
                    trade_type = if 0 == unit {String::from("-")}else{String::from("买入")};
                }

                // 计算在以前买入 到现在的收益（暂时不考虑手续费）
                let (_cost, _sell) = self.compute_earnings(&hold_detail, &net_worth,hold);
                //println!("{}", format!("->结算[{}]收益,持有份额:{},持有总市值:{},已套现额:{},总成本价:{},收益率{}%------", item.date.clone().unwrap(), hold, (_sell - cash_out), cash_out, _cost, if _cost.is_zero() { Decimal::zero() } else { ((_sell - _cost) / _cost * Decimal::from(100)).round_dp(5) }));
                // 当前收益 = 已套现额+持有总市值
                let income:Decimal =  _sell+cash_out;
                let _result = FundIncomeVO{
                    date: item.date.clone(),
                    net_worth: Some(net_worth.clone()),
                    rise_rate: item.equityReturn.clone().unwrap().to_f64(),
                    rise: Some(gains.clone()),
                    trade_type: Some(trade_type),
                    trade_share: Some(real_trade_share),
                    hold_share: Some(hold),
                    hold_value: Some(_sell),
                    cash_out: Some(cash_out),
                    cost: Some(_cost),
                    sell: Some(income),
                    earning_rate: Some(if _cost.is_zero() { 0.0 } else { ((income - _cost) / _cost * Decimal::from(100)).round_dp(5).to_f64().unwrap() })
                };
                result.push(_result);
                continue;
            }
            if equity_return < fall {
                // 下跌的趋势 正数为买入，负数为卖出
                let unit: u64 = self.compute_units(equity_return.clone(), fall.clone(), buy.abs() as u32);
                let mut trade_type:String = String::new();
                if buy >= 0 {
                    // 给予买入，并更新持有份额
                    real_trade_share = unit;
                    hold = self.buy_funds(unit, &net_worth, &mut hold_detail, hold);
                    trade_type = if 0 == unit {String::from("-")}else{String::from("买入")};
                } else {
                    // 给予卖出，并更新套现总额和份额
                    (hold, cash_out,real_trade_share) = self.sell_funds(unit, &net_worth, cash_out.clone(), hold);
                    trade_type = if 0 == real_trade_share {String::from("-")}else{String::from("赎回")};
                }
                let (_cost, _sell) = self.compute_earnings(&mut hold_detail, &net_worth,hold);
                //println!("{}", format!("->结算[{}]收益,持有份额:{},持有总市值:{},已套现额:{},总成本价:{},收益率{}%------", item.date.clone().unwrap(), hold, (_sell - cash_out), cash_out, _cost, if _cost.is_zero() { Decimal::zero() } else { ((_sell - _cost) / _cost * Decimal::from(100)).round_dp(5) }));
                // 当前收益 = 已套现额+持有总市值
                let income:Decimal =  _sell+cash_out;
                let _result = FundIncomeVO{
                    date: item.date.clone(),
                    net_worth: Some(net_worth.clone()),
                    rise_rate: item.equityReturn.clone().unwrap().to_f64(),
                    rise: Some(gains.clone()),
                    trade_type: Some(trade_type),
                    trade_share: Some(real_trade_share),
                    hold_share: Some(hold),
                    hold_value: Some(_sell),
                    cash_out: Some(cash_out),
                    cost: Some(_cost),
                    sell: Some(income),
                    earning_rate: Some(if _cost.is_zero() { 0.0 } else { ((income - _cost) / _cost * Decimal::from(100)).round_dp(5).to_f64().unwrap() })
                };
                result.push(_result);
                continue;
            }
            let (_cost, _sell) = self.compute_earnings(&mut hold_detail, &net_worth,hold);
            //println!("{}", format!("->结算[{}]收益,持有份额:{},持有总市值:{},已套现额:{},总成本价:{},收益率{}%------", item.date.clone().unwrap(), hold, (_sell - cash_out), cash_out, _cost, if _cost.is_zero() { Decimal::zero() } else { ((_sell - _cost) / _cost * Decimal::from(100)).round_dp(5) }));
            // 今日无交易
            // 当前收益 = 已套现额+持有总市值
            let income:Decimal =  _sell+cash_out;
            let _result = FundIncomeVO{
                date: item.date.clone(),
                net_worth: Some(net_worth.clone()),
                rise_rate: item.equityReturn.clone().unwrap().to_f64(),
                rise: Some(gains.clone()),
                trade_type: Some(String::from("-")),
                trade_share: Some(real_trade_share),
                hold_share: Some(hold),
                hold_value: Some(_sell),
                cash_out: Some(cash_out),
                cost: Some(_cost),
                sell: Some(income),
                earning_rate: Some(if _cost.is_zero() { 0.0 } else { ((income - _cost) / _cost * Decimal::from(100)).round_dp(5).to_f64().unwrap() })
            };
            result.push(_result);
        }
        return Ok(result);
    }

    /// 按照价格
    pub fn calculate_income_by_price(&self,arg: &FundSettingDTO) -> Result<Vec<FundIncomeVO>> {
        let mut result:Vec<FundIncomeVO> = Vec::new();
        // 当前持有信息
        let mut hold_detail: HashMap<Decimal, u64> = HashMap::new();
        // 总套现额
        let mut cash_out: Decimal = Decimal::from(0);
        // 持有份额（买入、卖出需要更新），用于校验在卖出时，判断是否充足，以及时刻计算总市值（持有份额*当前单位市值）
        let mut hold: u64 = 0;
        // 本次实际实际交易份额（在持有不足的情况下，卖出只能全部卖出）
        let mut real_trade_share: u64 = 0;

        let start_date:i64 = arg.start_date.clone().unwrap();
        let end_date:i64 = arg.end_date.clone().unwrap();

        // 参数载入
        let vec = FUND_GAINS.lock().unwrap();
        let rise: Decimal = arg.rise.clone().unwrap();
        let buy: i32 = arg.buy.clone().unwrap();
        let fall: Decimal = arg.fall.clone().unwrap();
        let sell: i32 = arg.sell.clone().unwrap();

        // 初始持有信息
        let init_share: u64 = arg.init_share.clone().unwrap();
        let init_net_worth: Decimal = arg.init_net_worth.clone().unwrap();
        hold = self.buy_funds(init_share, &init_net_worth, &mut hold_detail, hold);
        for item in vec.to_vec() {
            let _current:i64 = item.x.clone().unwrap();
            if _current < start_date || _current > end_date {
                // 不在时间范围内，跳过不予处理
                continue;
            }

            // 当日涨幅 净值
            let net_worth: Decimal = item.y.clone().unwrap();
            let gains: Decimal = item.gains.clone().unwrap();
            let equity_return: Decimal = item.equityReturn.clone().unwrap();
            //println!("{}", format!("\n-----{:?},净值:{:?},涨幅:{:?}------", item.date.clone().unwrap(), net_worth, gains.round_dp(5)));
            if gains >= rise {
                // 上涨的趋势 正数为卖出，负数为买入
                let unit: u64 = self.compute_units(gains.clone(), rise.clone(), sell.abs() as u32);
                // 本次交易类型
                let mut trade_type:String = String::new();
                if sell > 0 {
                    // 给予卖出
                    (hold, cash_out,real_trade_share) = self.sell_funds(unit, &net_worth, cash_out.clone(), hold);
                    trade_type = if 0 == real_trade_share {String::from("-")}else{String::from("赎回")};
                } else {
                    // 给予买入
                    real_trade_share = unit;
                    hold = self.buy_funds(unit, &net_worth, &mut hold_detail, hold);
                    trade_type = if 0 == unit {String::from("-")}else{String::from("买入")};
                }

                // 计算在以前买入 到现在的收益（暂时不考虑手续费）
                let (_cost, _sell) = self.compute_earnings(&hold_detail, &net_worth,hold);
                //println!("{}", format!("->结算[{}]收益,持有份额:{},持有总市值:{},已套现额:{},总成本价:{},收益率{}%------", item.date.clone().unwrap(), hold, (_sell - cash_out), cash_out, _cost, if _cost.is_zero() { Decimal::zero() } else { ((_sell - _cost) / _cost * Decimal::from(100)).round_dp(5) }));
                // 当前收益 = 已套现额+持有总市值
                let income:Decimal =  _sell+cash_out;
                let _result = FundIncomeVO{
                    date: item.date.clone(),
                    net_worth: Some(net_worth.clone()),
                    rise_rate: item.equityReturn.clone().unwrap().to_f64(),
                    rise: Some(gains.clone()),
                    trade_type: Some(trade_type),
                    trade_share: Some(real_trade_share),
                    hold_share: Some(hold),
                    hold_value: Some(_sell),
                    cash_out: Some(cash_out),
                    cost: Some(_cost),
                    sell: Some(income),
                    earning_rate: Some(if _cost.is_zero() { 0.0 } else { ((income - _cost) / _cost * Decimal::from(100)).round_dp(5).to_f64().unwrap()})
                };
                result.push(_result);
                continue;
            }
            if gains < fall {
                // 下跌的趋势 正数为买入，负数为卖出
                let unit: u64 = self.compute_units(gains.clone(), fall.clone(), buy.abs() as u32);
                let mut trade_type:String = String::new();
                if buy >= 0 {
                    // 给予买入，并更新持有份额
                    real_trade_share = unit;
                    hold = self.buy_funds(unit, &net_worth, &mut hold_detail, hold);
                    trade_type = if 0 == unit {String::from("-")}else{String::from("买入")};
                } else {
                    // 给予卖出，并更新套现总额和份额
                    (hold, cash_out,real_trade_share) = self.sell_funds(unit, &net_worth, cash_out.clone(), hold);
                    trade_type = if 0 == real_trade_share {String::from("-")}else{String::from("赎回")};
                }

                let (_cost, _sell) = self.compute_earnings(&mut hold_detail, &net_worth,hold);
                //println!("{}", format!("->结算[{}]收益,持有份额:{},持有总市值:{},已套现额:{},总成本价:{},收益率{}%------", item.date.clone().unwrap(), hold, (_sell - cash_out), cash_out, _cost, if _cost.is_zero() { Decimal::zero() } else { ((_sell - _cost) / _cost * Decimal::from(100)).round_dp(5) }));
                // 当前收益 = 已套现额+持有总市值
                let income:Decimal =  _sell+cash_out;
                let _result = FundIncomeVO{
                    date: item.date.clone(),
                    net_worth: Some(net_worth.clone()),
                    rise_rate: item.equityReturn.clone().unwrap().to_f64(),
                    rise: Some(gains.clone()),
                    trade_type: Some(trade_type),
                    trade_share: Some(real_trade_share),
                    hold_share: Some(hold),
                    hold_value: Some(_sell),
                    cash_out: Some(cash_out),
                    cost: Some(_cost),
                    sell: Some(income),
                    earning_rate: Some(if _cost.is_zero() { 0.0 } else { ((income - _cost) / _cost * Decimal::from(100)).round_dp(5).to_f64().unwrap()})
                };
                result.push(_result);
                continue;
            }
            let (_cost, _sell) = self.compute_earnings(&mut hold_detail, &net_worth,hold);
            //println!("{}", format!("->结算[{}]收益,持有份额:{},持有总市值:{},已套现额:{},总成本价:{},收益率{}%------", item.date.clone().unwrap(), hold, (_sell - cash_out), cash_out, _cost, if _cost.is_zero() { Decimal::zero() } else { ((_sell - _cost) / _cost * Decimal::from(100)).round_dp(5) }));
            // 今日无交易
            // 当前收益 = 已套现额+持有总市值
            let income:Decimal =  _sell+cash_out;
            let _result = FundIncomeVO{
                date: item.date.clone(),
                net_worth: Some(net_worth.clone()),
                rise_rate: item.equityReturn.clone().unwrap().to_f64(),
                rise: Some(gains.clone()),
                trade_type: Some(String::from("-")),
                trade_share: Some(real_trade_share),
                hold_share: Some(hold),
                hold_value: Some(_sell),
                cash_out: Some(cash_out),
                cost: Some(_cost),
                sell: Some(income),
                earning_rate: Some(if _cost.is_zero() { 0.0 } else { ((income - _cost) / _cost * Decimal::from(100)).round_dp(5).to_f64().unwrap()})
            };
            result.push(_result);
        }
        return Ok(result);
    }

    /// 计算上涨或者下跌后，应该买入的量
    /// net_worth_equity 当日涨幅变化（涨幅比，或者涨幅量）
    /// unit 买卖步长
    pub fn compute_units(&self, net_worth_equity: Decimal, rise_fall: Decimal, unit: u32) -> u64 {
        // 计算有多少段上涨或者下跌，以确定买入或者卖出多少笔
        let change_segment: u32 = net_worth_equity.div(rise_fall).trunc().abs().to_u32().unwrap();
        // 计算买入或者卖出笔数
        (change_segment * unit) as u64
    }

    /// 买入基金
    /// buy 买入份额参数
    /// net_worth 当日净值
    /// hold_detail 持有明细
    /// hold 持有总份额
    /// 返回时，计算后的持有份额（原持有+最新买入的）
    pub fn buy_funds(&self, buy: u64, net_worth: &Decimal, hold_detail: &mut HashMap<Decimal, u64>, hold: u64) -> u64 {
        if 0 == buy{
            return hold;
        }

        //println!("拟买入{}", buy);
        if hold_detail.contains_key(net_worth) {
            // 原持有份额
            let tranche = hold_detail.get(net_worth).unwrap();
            // 加仓
            hold_detail.insert(*net_worth, tranche + buy);
        } else {
            hold_detail.insert(*net_worth, buy);
        }
        hold + buy
    }

    /// 卖出基金
    /// sell 拟卖出份额参数
    /// net_worth 当日净值
    /// cash_out 总套现额
    /// hold 持有总份额
    /// 返回时，第一位为持有，第二位为套现总额，第三位为实际交易份额
    pub fn sell_funds(&self, sell: u64, net_worth: &Decimal, cash_out: Decimal, hold: u64) -> (u64, Decimal,u64) {
        //println!("拟卖出{}", sell);
        // 给予卖出
        // 检查是否充足
        if hold <= sell as u64 {
            //println!("干啥勒，份额不足，清空持有的份额{}",hold);
            return (0, cash_out.add(net_worth.mul(Decimal::from(hold))),hold);
        }
        // 给予卖出 并 添加到总套现中
        ((hold - sell), cash_out.add(net_worth.mul(Decimal::from(sell))),sell)
    }

    /// 计算持仓收益
    pub fn compute_earnings(&self, map: &HashMap<Decimal, u64>, net_worth: &Decimal,hold:u64) -> (Decimal, Decimal) {
        // 持有总成本价
        let mut cost: Decimal = Decimal::from(0);
        // 持有总市值
        let sell: Decimal = (Decimal::from(hold)).mul(net_worth);
        for (key, value) in map.iter() {
            // 持有份额
            let number: Decimal = Decimal::from(*value);
            cost = cost.add(number.mul(key));
        }
        (cost, sell)
    }

    /// 将计算结果输出到excel
    pub fn out_excel(&self, arg: &FundSettingDTO) -> Result<String>{
        let save_path_op = arg.save_path.clone();
        if save_path_op.is_none() {
            return Err(Error::from("缺少保存路径..."));
        }
        let save_path:String = save_path_op.unwrap();
        let result= self.calculate_income(arg);
        if result.is_err() {
            return Err(result.unwrap_err())
        }
        let rows:Vec<FundIncomeVO> = result.unwrap();
        let mut wb:Workbook = Workbook::create(format!("{}/预测结果.xlsx",save_path).as_str());
        let mut sheet:Sheet = wb.create_sheet("收益预测");
        // 设置列宽
        sheet.add_column(Column { width: 12.0 });
        sheet.add_column(Column { width: 15.0 });
        sheet.add_column(Column { width: 15.0 });
        sheet.add_column(Column { width: 15.0 });
        sheet.add_column(Column { width: 15.0 });
        sheet.add_column(Column { width: 15.0 });
        sheet.add_column(Column { width: 15.0 });
        sheet.add_column(Column { width: 15.0 });
        sheet.add_column(Column { width: 15.0 });
        sheet.add_column(Column { width: 15.0 });
        sheet.add_column(Column { width: 15.0 });
        wb.write_sheet(&mut sheet, |sheet_writer| {
            let sw = sheet_writer;
            // 写入标题行
            sw.append_row(row!["日期","净值", "净值涨幅", "涨幅", "交易类型", "交易份额","持有份额", "持有总市值", "已赎回", "总成本价", "累计收益"]);
            for item in rows {
                sw.append_row(row![item.date.unwrap(),item.net_worth.unwrap().to_string(),item.rise_rate.unwrap(),item.rise.unwrap().to_string(),item.trade_type.unwrap(),
                item.trade_share.unwrap().to_string(),item.hold_share.unwrap().to_string(),item.hold_value.unwrap().to_string(),item.cash_out.unwrap().to_string(),item.cost.unwrap().to_string(),item.earning_rate.unwrap().to_string()
                ]);
            }
            Ok(())
        }).expect("write excel error!");
        wb.close().expect("close excel error!");
        return Ok(String::from("导出成功"))
    }
}
