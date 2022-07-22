mod routes;

use once_cell::sync::Lazy;
use reqwest::Client;
use std::{collections::HashMap, error::Error};
use tracing::{info, warn};
use tracing_subscriber;
use warp::{Filter, http::StatusCode, reject::MethodNotAllowed};

pub static HTTP: Lazy<Client> = Lazy::new(Client::new);

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    let hook = warp::post()
        .and(warp::path!("message"))
        .and(warp::body::json::<HashMap<String, serde_json::Value>>())
        .and_then(routes::message)
        .recover(handle_rejection);

    info!("Starting...");

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not set CTRL-C handler");
        warn!("Received Termination Signal...");
        std::process::exit(0)
    });

    warp::serve(hook).run(([127, 0, 0, 1], 80)).await;

    Ok(())
}

async fn handle_rejection(err: warp::Rejection) -> Result<impl warp::Reply, std::convert::Infallible> {
  if err.is_not_found() {
    Ok(warp::reply::with_status("Could not find that route", StatusCode::NOT_FOUND))
  } else if let Some(_) = err.find::<MethodNotAllowed>() {
    Ok(warp::reply::with_status("METHOD_NOT_ALLOWED", StatusCode::METHOD_NOT_ALLOWED))
  } else {
    eprintln!("Unhandled rejection: {:?}", err);
    Ok(warp::reply::with_status("INTERNAL_SERVER_ERROR", StatusCode::INTERNAL_SERVER_ERROR))
  }
}