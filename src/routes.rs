use crate::HTTP;
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
    let message = &payload["message"];
    let embed = serde_json::json!({
      "title": "Hello world!",
      "description": message.as_str()
    });

    let body = serde_json::json!({
        "embeds": [embed]
    });

    let _ = HTTP.post(webhook_url).json(&body).send().await;

    Ok(format!("{}", "OK"))
}
