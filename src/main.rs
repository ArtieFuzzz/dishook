mod routes;

use once_cell::sync::Lazy;
use reqwest::Client;
use std::{collections::HashMap, error::Error};
use tracing::info;
use tracing_subscriber;
use warp::Filter;

pub static HTTP: Lazy<Client> = Lazy::new(Client::new);

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    let hook = warp::post()
        .and(warp::path!("message"))
        .and(warp::body::json::<HashMap<String, serde_json::Value>>())
        .and_then(routes::message);

    info!("Starting...");

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not set CTRL-C handler");
        info!("Received Termination Signal...");
        std::process::exit(0)
    });

    warp::serve(hook).run(([127, 0, 0, 1], 80)).await;

    Ok(())
}
