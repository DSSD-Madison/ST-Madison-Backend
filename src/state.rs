use dotenvy::dotenv;
use duckdb::{Connection, params};
use std::env;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Mutex<Connection>>,
}

impl AppState {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let db = initialize_database()?;

        Ok(Self {
            db: Arc::new(Mutex::new(db)),
        })
    }
}

const GOLD_VIEWS: &[(&str, &str)] = &[
    ("alder_districts", "fact_alder_districts.parquet"),
    ("area_plans", "fact_area_plans.parquet"),
    ("sites", "fact_sites.parquet"),
];

const SILVER_VIEWS: &[(&str, &str)] = &[
    ("parcels", "fact_parcels.parquet"),
    ("streets", "fact_streets.parquet"),
    ("tax_roll", "fact_tax_roll.parquet"),
];

fn initialize_database() -> duckdb::Result<Connection> {
    dotenv().ok();
    let conn = Connection::open_in_memory()?;

    let (gcs_key_id, gcs_secret) = match (env::var("GCS_KEY_ID"), env::var("GCS_SECRET")) {
        (Ok(key), Ok(secret)) => (key, secret),
        _ => todo!(),
    };

    conn.execute("INSTALL httpfs;", [])?;
    conn.execute("LOAD httpfs;", [])?;
    conn.execute(
        r#"
        CREATE OR REPLACE PERSISTENT SECRET gcs_credentials (
            TYPE gcs,
            KEY_ID ?,
            SECRET ?
        );
        "#,
        params![gcs_key_id, gcs_secret],
    )?;

    conn.execute_batch(
        r#"
        CREATE SCHEMA IF NOT EXISTS gold;
        CREATE SCHEMA IF NOT EXISTS silver;
        "#,
    )?;

    let mut view_queries: Vec<String> = Vec::new();
    for view in SILVER_VIEWS {
        view_queries.push(format!(
            "CREATE OR REPLACE VIEW silver.{} AS SELECT * FROM 'gs://stmsn-silver/{}';",
            view.0, view.1
        ));
    }

    for view in GOLD_VIEWS {
        view_queries.push(format!(
            "CREATE OR REPLACE VIEW gold.{} AS SELECT * FROM 'gs://stmsn-gold/{}';",
            view.0, view.1
        ));
    }
    conn.execute_batch(&view_queries.join(" "))?;

    Ok(conn)
}

pub enum StateError {
    StateInitializationError,
    DatabaseInitializationError,
    EnvError,
}
