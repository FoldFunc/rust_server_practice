mod database;
mod handlers;
mod handlersstocks;
use actix_web::{App, HttpServer, web};
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = database::singleton_database_instance_launcher().await;
    database::database_table_creation_function_users(&pool)
        .await
        .expect("Error in database_table_creation_function_users");

    database::database_table_creation_function_token(&pool)
        .await
        .expect("Error in database_table_creation_function_token");
    database::database_check_for_outtime_tokens(&pool)
        .await
        .expect("Error in database_check_for_outtime_tokens");
    database::database_table_creation_function_crypto(&pool)
        .await
        .expect("Error in database_check_for_outtime_crypto");

    database::database_table_creation_function_whitelist(&pool)
        .await
        .expect("Error in database_check_for_outtime_whitelist");
    database::database_table_creation_function_portfolios(&pool)
        .await
        .expect("Error in database_check_for_outtime_portfolio");

    let addr = "localhost:8080";
    println!("Server running on http://{}", addr);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone())) // <- Inject pool
            .route("/api/register", web::post().to(handlers::register_handler))
            .route("/api/login", web::post().to(handlers::login_handler))
            .route("/api/logout", web::post().to(handlers::logout_handler))
            .route("/api/getroot", web::post().to(handlers::create_a_root))
            .route("/api/addportfolio", web::post().to(handlers::addportfolio))
            .route(
                "/api/deleteportfolio",
                web::post().to(handlers::deleteportfolio),
            )
            .route(
                "/api/fetch/cryptonames",
                web::post().to(handlersstocks::fetchstocknames),
            )
            .route(
                "/api/fetch/cryptoprices",
                web::post().to(handlersstocks::fetchstockprices),
            )
            .route(
                "/api/fetch/cryptospecific",
                web::post().to(handlersstocks::fetchstockspecific),
            )
            .route(
                "/api/middlewear/changeprice",
                web::post().to(handlers::change_price_handler),
            )
            .route(
                "/api/root/addcrypto",
                web::post().to(handlers::create_crypto),
            )
            .route(
                "/api/root/removecrypto",
                web::post().to(handlers::removecrypto),
            )
    })
    .bind(addr)?
    .run()
    .await
}
