use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct WebhookMessage {
    content: String,
}

pub async fn message(
    payload: HashMap<String, serde_json::Value>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let message = &payload["message"];

    let body = WebhookMessage {
        content: message.to_string(),
    };

    let _ = reqwest::Client::new().post("https://discord.com/api/webhooks/999249118788599878/SFy8Lohs_ly56dPNc892R-UatJwtNNaZypJydZKoft9ZCVX5bbF_d_-PrmkYlc09--Vc")
        .json(&body)
        .send()
        .await;

    Ok(format!("{}", "OK"))
}
