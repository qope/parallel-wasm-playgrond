use serde::Deserialize;
use wasm_bindgen::prelude::*;

#[derive(Deserialize, Debug)]
struct IP {
    origin: String,
}

#[wasm_bindgen]
pub async fn ip_get() -> String {
    let client = reqwest::Client::new();
    let body = client
        .get("https://httpbin.org/get")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let ip: IP = serde_json::from_str(&body).unwrap();

    ip.origin
}
