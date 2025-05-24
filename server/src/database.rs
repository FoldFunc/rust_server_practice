use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions; // Add this
pub async fn database_table_creation_function_token(pool: &PgPool) -> Result<(), sqlx::Error> {
    let query = r#"
        CREATE TABLE IF NOT EXISTS token(
            token VARCHAR(255) NOT NULL,
            owner VARCHAR(255) NOT NULL,
            created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
        );
    "#;

    sqlx::query(query).execute(pool).await?;
    println!("Created token");
    Ok(())
}
pub async fn database_table_creation_function_portfolios(pool: &PgPool) -> Result<(), sqlx::Error> {
    let query = r#"
        CREATE TABLE IF NOT EXISTS portfolios(
        id SERIAL PRIMARY KEY,
        money INT4 NOT NULL,
        owner VARCHAR(255),
        name VARCHAR(255),
        assets VARCHAR(255),
        password VARCHAR(255),
        created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
        );
    "#;

    sqlx::query(query).execute(pool).await?;
    println!("Created portfolios");
    Ok(())
}
pub async fn database_table_creation_function_crypto(pool: &PgPool) -> Result<(), sqlx::Error> {
    let query = r#"
        CREATE TABLE IF NOT EXISTS crypto(
            id SERIAL PRIMARY KEY,
            name VARCHAR(255) NOT NULL,
            creator VARCHAR(255) NOT NULL,
            price INT4 NOT NULL,
            created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
        );
    "#;

    sqlx::query(query).execute(pool).await?;
    println!("Created crypto");
    Ok(())
}
pub async fn database_table_creation_function_users(pool: &PgPool) -> Result<(), sqlx::Error> {
    let query = r#"
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            email VARCHAR(255) UNIQUE NOT NULL,
            password VARCHAR(255) NOT NULL,
            created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
        );
    "#;

    sqlx::query(query).execute(pool).await?;
    println!("Created users");
    Ok(())
}
pub async fn database_table_creation_function_whitelist(pool: &PgPool) -> Result<(), sqlx::Error> {
    let query = r#"
        CREATE TABLE IF NOT EXISTS whitelist(
            id SERIAL PRIMARY KEY,
            email VARCHAR(255) UNIQUE NOT NULL,
            created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
        );
    "#;

    sqlx::query(query).execute(pool).await?;
    println!("Created whitelist");
    Ok(())
}

pub async fn singleton_database_instance_launcher() -> PgPool {
    let database_url = "postgres://gsliv:2010@localhost/db1";

    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(database_url)
        .await
        .expect("Failed to connect to DB");

    println!("Connected to a database");
    pool
}

pub async fn database_check_for_outtime_tokens(pool: &PgPool) -> Result<(), sqlx::Error> {
    // Delete tokens older than 1 day
    let query = r#"
        DELETE FROM token
        WHERE created_at < NOW() - INTERVAL '1 day'
    "#;

    sqlx::query(query).execute(pool).await?;

    Ok(())
}
