mod config;
mod service;

use config::ApiConfig;
use futures::{stream, StreamExt};
use json_parser::{Attrs, RequestBody};
use reqwest::Client;
mod json_parser;
use service::{ApiClient, Requests};
use tokio;

const PARALLEL_REQUESTS: usize = 2;
const CHUNK_SIZE: usize = 2;

#[tokio::main]
async fn main() {
    let request = Attrs::new("./attributes.json");
    let client = Client::new();
    config::ApiConfig::init();

    let bodies = stream::iter(request.list)
        .chunks(CHUNK_SIZE)
        .map(|params| {
            let client = client.clone();
            tokio::spawn(async move {
                let body: String = RequestBody { attributes: params }.into();
                let api = ApiClient::new(client);
                let headers = ApiClient::common_headers(&ApiConfig::get(
                    config::ConfigKeys::ApiToken.to_string(),
                ));
                api.send_post(
                    &ApiConfig::get(config::ConfigKeys::ApiUrl.to_string()),
                    body,
                    headers,
                )
                .await
            })
        })
        .buffer_unordered(PARALLEL_REQUESTS);

    bodies
        .for_each(|b| async {
            match b {
                Ok(Ok(b)) => println!("Got {}", String::from_utf8(b.to_vec()).unwrap()),
                Ok(Err(e)) => eprintln!("Got a reqwest::Error: {}", e),
                Err(e) => eprintln!("Got a tokio::JoinError: {}", e),
            }
        })
        .await;
}
