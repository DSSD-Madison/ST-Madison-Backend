# Models

A model is a struct that mirrors our database table structure.

This is the bridge between the database and our application.

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;


pub struct User {
    pub id: Uuid,                   // Maps to UUID
    pub username: String,           // Maps to VARCHAR(255)
    pub email: String,              // Maps to VARCHAR(255)
    pub password_hash: String,      // Maps to VARCHAR(255)
    pub bio: Option<String>,        // Maps to TEXT (nullable)
    pub created_at: DateTime<Utc>,  // Maps to TIMESTAMP WITH TIME ZONE
}
```
