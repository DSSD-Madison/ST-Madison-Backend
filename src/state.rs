use dotenvy::dotenv;
use duckdb::Connection;
use std::env;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Mutex<Connection>>,
}

impl AppState {
    pub fn new(database_url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        dotenv().ok();

        let db_path =
            env::var("DATABASE_PATH").unwrap_or_else(|_| "./data/local.duckdb".to_string());

        std::fs::create_dir_all("data").ok();

        let conn = Connection::open(&db_path)?;

        conn.execute_batch("LOAD httpfs;")?;

        let gcs_key_id = env::var("GCS_KEY_ID")?;
        let gcs_secret = env::var("GCS_SECRET")?;

        conn.execute_batch(&format!(
            "CREATE TEMPORARY SECRET gcs_secret (
                    TYPE gcs,
                    KEY_ID '{}',
                    SECRET '{}'
                );",
            gcs_key_id, gcs_secret
        ))?;

        Ok(Self {
            db: Arc::new(Mutex::new(conn)),
        })
    }
}
