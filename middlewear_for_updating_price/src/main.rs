use reqwest::Client;
use serde::Serialize;
use sqlx::Error;
use sqlx::{PgPool, Row, postgres::PgPoolOptions};
use std::collections::HashSet;
use std::time::Duration;

#[derive(Serialize, Clone)]
struct ToSent {
    name: String,
    secret_key: String,
}

async fn singleton_database_instance_launcher() -> PgPool {
    let database_url = "postgres://gsliv:2010@localhost/db1";
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(database_url)
        .await
        .expect("Failed to connect to DB");
    println!("Connected to db");
    pool
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_pool = singleton_database_instance_launcher().await;
    let active_cryptos = HashSet::new();

    loop {
        let latest_cryptos: HashSet<String> = get_cryptos(&db_pool).await?.into_iter().collect();

        // Spawn tasks only for new cryptos
        for name in latest_cryptos.clone().difference(&active_cryptos) {
            let name = name.clone();
            active_cryptos.clone().insert(name.clone());
            tokio::spawn(async move {
                if let Err(e) = send_request_loop(name).await {
                    eprintln!("Error updating crypto: {}", e);
                }
            });
        }

        // Poll for changes every 10 seconds
        tokio::time::sleep(Duration::from_secs(10)).await;
    }
}

async fn send_request_loop(name: String) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let secret_key = ToSent {
        name: name.clone(),
        secret_key: "secret_no_tell".to_string(),
    };

    loop {
        let res = client
            .post("http://localhost:8080/api/middlewear/changeprice")
            .json(&secret_key)
            .send()
            .await?;

        println!("Updated {}: {}", name, res.status());
        println!("Response: {}", res.text().await?);

        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

async fn get_cryptos(pool: &PgPool) -> Result<Vec<String>, Error> {
    let rows = sqlx::query(
        r#"
        SELECT name FROM crypto
        "#,
    )
    .fetch_all(pool)
    .await?;

    let names: Vec<String> = rows
        .into_iter()
        .map(|row| row.get::<String, _>("name"))
        .collect();

    Ok(names)
}
