use axum::{Router, routing::get};
use st_madison_backend::{
    handlers::{health::health_check, property::get_property_by_address, land_efficiency::get_land_efficiency_metrics},
    state::AppState,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let app_state = AppState::new().unwrap();

    println!("connected to database");

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/property/{address}", get(get_property_by_address))
        .route("/land-efficiency", get(get_land_efficiency_metrics))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Server is running on http://0.0.0.0:3000");
    axum::serve(listener, app).await?;
    Ok(())
}
