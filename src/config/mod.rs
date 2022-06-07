use dotenv::dotenv;
use std::env;

pub struct ApiConfig {}

#[derive(strum_macros::Display)]
pub enum ConfigKeys {
    ApiUrl,
    ApiToken,
}
impl ApiConfig {
    pub fn init() {
        dotenv().ok();
    }
    pub fn get(key: String) -> String {
        env::var(key).unwrap()
    }
}
