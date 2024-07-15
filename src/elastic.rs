use base64::prelude::{Engine, BASE64_STANDARD};
use reqwest::{header::AUTHORIZATION, Client};
use std::error::Error;
use std::sync::Arc;

pub async fn send_to_elasticsearch(
    client: Client,
    url: Arc<String>,
    user: Arc<Option<String>>,
    password: Arc<Option<String>>,
    json_entry: String,
) -> Result<(), Box<dyn Error>> {
    let index_url = format!("{}/app_logs/_doc", &url);

    // create a Reqwest request builder
    let mut req_builder = client
        .post(&index_url)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(json_entry);

    if let (Some(user), Some(password)) = (&*user, &*password) {
        let auth = reqwest::header::HeaderValue::from_str(&format!(
            "Basic {}",
            BASE64_STANDARD.encode(format!("{}:{}", user, password))
        ))?;
        req_builder = req_builder.header(AUTHORIZATION, auth);
    }

    let response = req_builder.send().await?;

    if !response.status().is_success() {
        return Err(format!("Failed to send to Elasticsearch: {}", response.status()).into());
    }

    Ok(())
}
