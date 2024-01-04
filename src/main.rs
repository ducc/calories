use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, collections::HashMap};

mod models;
use models::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    tracing_subscriber::fmt::init();

    let res = rxing::helpers::detect_in_file("orange.jpg", None).expect("detecting barcode");
    let barcode = res.getText();

    println!("barcode: {}", barcode);

    let resp = reqwest::get(format!("https://world.openfoodfacts.org/api/v2/product/{}.json", barcode))
        .await?
        .json::<Root>()
        .await?;

    println!("{:#?}", resp);

    // let app = Router::new()
    //     .route("/", get(root))
    //     .route("/products", get(get_products));

    // let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // tracing::debug!("listening on {}", addr);
    // axum::Server::bind(&addr)
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();

    Ok(())
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}
