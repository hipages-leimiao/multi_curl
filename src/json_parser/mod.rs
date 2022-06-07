use serde::{Deserialize, Serialize};
use std::fs;
#[derive(Debug, Serialize, Deserialize)]
#[serde()]
pub struct RequestBody {
    pub(crate) attributes: Vec<Attr>,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde()]
pub struct Attr {
    external_id: String,
    stripe_account_status: String,
}
pub(crate) struct Attrs {
    pub list: Vec<Attr>,
}
impl Attrs {
    pub fn new(path: &str) -> Self {
        let data = fs::read_to_string(path).expect("Unable to read json file");
        data.into()
    }
}

impl From<RequestBody> for String {
    fn from(body: RequestBody) -> Self {
        serde_json::to_string(&body).unwrap()
    }
}

impl From<String> for Attrs {
    fn from(body: String) -> Attrs {
        let data = serde_json::from_str::<Vec<Attr>>(&body).expect("JSON was not well-formatted");
        Self { list: data }
    }
}
