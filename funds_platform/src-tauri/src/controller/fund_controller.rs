use crate::util::Result;
use crate::util::Error;
use crate::dto::fund_setting_dto::FundSettingDTO;
use crate::service::{CONTEXT, FUND_GAINS, FundService};
use crate::vo::fund_info_vo::FundInfoVO;
use crate::vo::fund_income_vo::FundIncomeVO;

#[tauri::command]
pub async fn query_fund_info(fund_code:&str) -> Result<FundInfoVO> {
    let funds_info = CONTEXT.fund_service.get_funds_info(fund_code).await;
    if funds_info.is_ok() {
        let fund = funds_info.clone().unwrap();
        CONTEXT.fund_service.get_fund_net_worth_trend(fund_code).await;
        let vec = FUND_GAINS.lock().unwrap().to_vec();
        if !vec.is_empty() {
            let start_date = vec.get(0).unwrap().x;
            let end_date = vec.get(vec.to_vec().len()-1).unwrap().x;
            let result = FundInfoVO{
                start_date:vec.get(0).unwrap().x,
                end_date:vec.get(vec.to_vec().len()-1).unwrap().x,
                ..fund
            };
            return Ok(result);
        }
    }
    return Err(funds_info.unwrap_err());
}

#[tauri::command]
pub async fn fund_calculate(param:FundSettingDTO) -> Result<Vec<FundIncomeVO>> {
    CONTEXT.fund_service.calculate_income(&param)
}