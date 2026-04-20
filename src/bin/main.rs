use axum::{Router, routing::get};
use st_madison_backend::{
    handlers::{health::health_check, parcel::get_parcel_by_address, property::get_property_by_address},
    state::AppState,
};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let app_state = AppState::new().unwrap();

    println!("connected to database");

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/property/{address}", get(get_property_by_address))
        .route("/parcel/{address}", get(get_parcel_by_address))
        .layer(cors)
        .with_state(app_state);

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("Server is running on http://{addr}");
    axum::serve(listener, app).await?;
    Ok(())
}
