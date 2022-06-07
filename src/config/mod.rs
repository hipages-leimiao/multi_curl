use dotenv::dotenv;
use std::env;

pub struct ApiConfig {
    pub url: String,
    pub token: String,
}

impl ApiConfig {
    pub fn new() -> Self {
        dotenv().ok();

        Self {
            url: env::var("API_URL").unwrap(),
            token: env::var("API_TOKEN").unwrap(),
        }
    }
}
