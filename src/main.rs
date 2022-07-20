mod routes;

use std::{collections::HashMap, error::Error};
use tracing::info;
use tracing_subscriber;
use warp::Filter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    let hook = warp::post()
      .and(warp::path!("message"))
      .and(warp::body::json::<HashMap<String, serde_json::Value>>())
      .and_then(routes::message);

    info!("Starting...");

    warp::serve(hook).run(([127, 0, 0, 1], 80)).await;

    Ok(())
}
