mod config;

use futures::{stream, StreamExt};
use json_parser::{Attrs, RequestBody};
use reqwest::{
    header::{CONTENT_TYPE, USER_AGENT},
    Client,
};
mod json_parser;
use tokio;

const PARALLEL_REQUESTS: usize = 2;
const CHUNK_SIZE: usize = 2;

#[tokio::main]
async fn main() {
    let request = Attrs::new("./attributes.json");
    let client = Client::new();

    let bodies = stream::iter(request.list)
        .chunks(CHUNK_SIZE)
        .map(|params| {
            let client = client.clone();
            let config = config::ApiConfig::new();
            tokio::spawn(async move {
                let body: String = RequestBody { attributes: params }.into();
                let resp = client
                    .post(config.url)
                    .body(body)
                    .bearer_auth(config.token)
                    .header(USER_AGENT, "rust-agent")
                    .header(CONTENT_TYPE, "application/json")
                    .send()
                    .await?;
                resp.bytes().await
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
