use crate::HTTP;
use crate::responses;
use std::collections::HashMap;
use std::env::var;

pub async fn message(
    payload: HashMap<String, serde_json::Value>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let webhook_url = if let Ok(url) = var("WEBHOOK_URL") {
        url
    } else {
      panic!("You must set the WEBHOOK_URL Env Variable")
    };

    let title = payload["title"].as_str().unwrap_or("Message");
    let message = payload["message"].as_str().unwrap_or("No Message...");
    let embed = serde_json::json!({
      "title": title,
      "description": message
    });

    let body = serde_json::json!({
        "embeds": [embed]
    });

    let _ = HTTP.post(webhook_url).json(&body).send().await;

    Ok(responses::success())
}
