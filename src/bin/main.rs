use std::env;

use axum::{Router, routing::get};
use st_madison_backend::{
    handlers::{health::health_check, property::get_property_by_address},
    state::AppState,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");

    let app_state = AppState::new(&database_url).unwrap();

    println!("connected to database");

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/property/{address}", get(get_property_by_address))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Server is running on http://0.0.0.0:3000");
    axum::serve(listener, app).await?;
    Ok(())
}
