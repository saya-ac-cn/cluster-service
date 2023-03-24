/// Config
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApplicationConfig {
    pub debug: bool,
    pub server_url: String,
    pub database_url: String,
    pub redis_url: String,
    pub log_dir: String,
    pub log_temp_size: String,
    pub log_pack_compress: String,
    pub log_rolling_type: String,
    pub log_level: String,
    pub log_chan_len: Option<usize>,
    pub jwt_secret: String,
    pub white_list_api: Vec<String>
}

impl Default for ApplicationConfig {
    fn default() -> Self {
        let yml_data = include_str!("../../application.yml");
        //load config
        let result: ApplicationConfig =
            serde_yaml::from_str(yml_data).expect("load config file fail");
        if result.debug {
            println!("[primary_server] load config:{:?}", result);
            println!("[primary_server] ///////////////////// Start On Debug Mode ////////////////////////////");
        } else {
            println!("[primary_server] ///////////////////// Start On Release Mode ////////////////////////////");
        }
        result
    }
}
