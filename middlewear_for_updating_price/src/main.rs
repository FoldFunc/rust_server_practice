use reqwest::Client;
use serde::Serialize;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::thread;
use std::time::Duration;

#[derive(Serialize)]
struct ToSent {
    name: String,
    secret_key: String,
}

async fn singleton_database_instance_launcher() -> PgPool {
    let database_url = "postgres://gsliv:2010@localhost/db1";
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(database_url) // <-- fix here
        .await
        .expect("Failed to connect to DB");
    println!("Connected to db");
    pool
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_pool = singleton_database_instance_launcher().await; // <-- await here

    let names: Vec<i32> = get_cryptos(&db_pool).await?;

    println!("Fetched crypto IDs: {:?}", names);

    let client = Client::new();
    let secret_key = ToSent {
        name: "Solana".to_string(),
        secret_key: "secret_no_tell".to_string(),
    };

    loop {
        let res = client
            .post("http://localhost:8080/api/root/changeprice")
            .json(&secret_key)
            .send()
            .await?;

        println!("Status: {}", res.status());
        println!("Response: {}", res.text().await?);
        thread::sleep(Duration::from_secs(1));
    }
}

async fn get_cryptos(pool: &PgPool) -> Result<Vec<i32>, sqlx::Error> {
    let rows = sqlx::query(
        r#"
        SELECT id FROM crypto
        "#,
    )
    .fetch_all(pool)
    .await?;

    let ids = rows.into_iter().map(|row| row.id).collect();
    println!("Fetched all crypto IDs");

    Ok(ids)
}
