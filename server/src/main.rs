mod database;
mod handlers;
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
        .expect("Error in database_check_for_outtime_tokens");
    let addr = "127.0.0.1:8080";
    println!("Server running on http://{}", addr);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone())) // <- Inject pool
            .route("/api/register", web::post().to(handlers::register_handler))
            .route("/api/login", web::post().to(handlers::login_handler))
            .route("/api/logout", web::post().to(handlers::logout_handler))
            .route(
                "/api/root/changeprice",
                web::post().to(handlers::change_price_handler),
            )
    })
    .bind(addr)?
    .run()
    .await
}
