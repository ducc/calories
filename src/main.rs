use chrono::{Duration, Utc, NaiveDate};

use mongodb::bson::doc;

pub mod entries;
mod models;

mod dateiter;
mod mongo;
mod nutracheck;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt::init();

    let mongo_client = mongo::Client::new_from_env().await?;

    let nc_client = nutracheck::Client::new_from_env().await?;

    let todays_date = Utc::now().date_naive();

    for date in dateiter::DateRange(todays_date - Duration::weeks(52), todays_date) {
        let entries = nc_client.entries(date).await.expect("getting entries");

        mongo_client.insert_entries(date, entries).await?;
    }

    // let res = rxing::helpers::detect_in_file("orange.jpg", None).expect("detecting barcode");
    // let barcode = res.getText();

    // println!("barcode: {}", barcode);

    // let resp = reqwest::get(format!("https://world.openfoodfacts.org/api/v2/product/{}.json", barcode))
    //     .await?
    //     .json::<Root>()
    //     .await?;

    // println!("{:#?}", resp);

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

