use crate::HTTP;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env::var;

#[derive(Serialize, Deserialize)]
struct WebhookMessage {
    content: String,
}

pub async fn message(
    payload: HashMap<String, serde_json::Value>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let webhook_url = if let Ok(url) = var("WEBHOOK_URL") {
        url
    } else {
      panic!("You must set the WEBHOOK_URL Env Variable")
    };
    let message = &payload["message"];

    let body = WebhookMessage {
        content: format!("{}", message.to_string()),
    };

    let _ = HTTP.post(webhook_url).json(&body).send().await;

    Ok(format!("{}", "OK"))
}
