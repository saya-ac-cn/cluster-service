use crate::error::{Error,Result};
use crate::entity::dto::login::LoginDTO;


#[tauri::command]
pub async fn login(arg: LoginDTO) -> Result<String> {
    println!("arg: {:?}", arg);
    //Ok(String::from("login success"))
    return Err(Error::from("查询文件失败!"));
}