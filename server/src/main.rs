use actix_web::cookie::{Cookie, SameSite};
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};
use cookie::time;
use serde::Deserialize;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions; // Add this
use uuid::Uuid;
async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[derive(Deserialize)]
struct RegisterDataStruct {
    email: String,
    password: String,
}

async fn register_handler(
    register_data: web::Json<RegisterDataStruct>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    if register_data.password.len() < 8 {
        return HttpResponse::BadRequest().body("Password too short");
    }
    if !register_data.email.contains("@") {
        return HttpResponse::BadRequest().body("Email not correct");
    }

    // Check if email exists
    let result = sqlx::query("SELECT 1 FROM users WHERE email = $1")
        .bind(&register_data.email)
        .fetch_optional(db_pool.get_ref())
        .await;

    if let Err(e) = result {
        eprintln!("Database error (select): {}", e);
        return HttpResponse::InternalServerError().body("Database error");
    }

    if result.unwrap().is_some() {
        return HttpResponse::Conflict().body("Email already registered");
    }

    // Insert new user
    let insert_result = sqlx::query("INSERT INTO users (email, password) VALUES ($1, $2)")
        .bind(&register_data.email)
        .bind(&register_data.password)
        .execute(db_pool.get_ref())
        .await;

    match insert_result {
        Ok(_) => HttpResponse::Ok().body("Register successful"),
        Err(e) => {
            eprintln!("Database error (insert): {:?}", e);
            HttpResponse::InternalServerError().body("Database issue")
        }
    }
}

#[derive(Deserialize)]
struct LoginDataStruct {
    email: String,
    password: String,
}
async fn login_handler(
    login_data: web::Json<LoginDataStruct>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let result = sqlx::query("SELECT 1 FROM users WHERE email = $1 AND password = $2")
        .bind(&login_data.email)
        .bind(&login_data.password)
        .fetch_optional(db_pool.get_ref())
        .await;

    match result {
        Ok(Some(_)) => {
            // Check if user already has a token
            let existing_token = sqlx::query("SELECT 1 FROM token WHERE owner = $1")
                .bind(&login_data.email)
                .fetch_optional(db_pool.get_ref())
                .await;

            match existing_token {
                Ok(Some(_)) => {
                    return HttpResponse::BadRequest().body("Already logged in");
                }
                Ok(None) => {
                    // Generate token
                    let token = Uuid::new_v4().to_string();

                    let insert_res =
                        sqlx::query("INSERT INTO token (token, owner) VALUES ($1, $2)")
                            .bind(&token)
                            .bind(&login_data.email)
                            .execute(db_pool.get_ref())
                            .await;

                    if let Err(e) = insert_res {
                        eprintln!("Token insert error: {}", e);
                        return HttpResponse::InternalServerError().body("Token insert error");
                    }

                    // Set cookie
                    let cookie = Cookie::build("auth", token)
                        .path("/")
                        .http_only(true)
                        .secure(false) // Set true in production
                        .same_site(SameSite::Lax)
                        .max_age(time::Duration::days(1))
                        .finish();

                    return HttpResponse::Ok().cookie(cookie).body("Login successful");
                }
                Err(e) => {
                    eprintln!("Error while checking for existing token: {}", e);
                    return HttpResponse::InternalServerError().body("Database error");
                }
            }
        }
        Ok(None) => HttpResponse::Unauthorized().body("Invalid email or password"),
        Err(e) => {
            eprintln!("Login DB error: {}", e);
            HttpResponse::InternalServerError().body("Database error")
        }
    }
}
async fn logout_handler(req: HttpRequest, db_pool: web::Data<PgPool>) -> impl Responder {
    // Try to get the "auth" cookie from the request
    println!("{:?}", req);
    if let Some(cookie) = req.cookie("auth") {
        let token_value = cookie.value();

        // Delete the token row from DB by token value
        let result = sqlx::query("DELETE FROM token WHERE token = $1")
            .bind(token_value)
            .execute(db_pool.get_ref())
            .await;

        match result {
            Ok(_) => {
                // Remove cookie on client side by setting a cookie with max_age = 0
                let expired_cookie = Cookie::build("auth", "")
                    .path("/")
                    .http_only(true)
                    .max_age(time::Duration::seconds(0))
                    .finish();

                HttpResponse::Ok().cookie(expired_cookie).body("Logged out")
            }
            Err(e) => {
                eprintln!("DB error on logout: {}", e);
                HttpResponse::InternalServerError().body("Database error")
            }
        }
    } else {
        HttpResponse::BadRequest().body("No auth cookie found")
    }
}

async fn database_table_creation_function_token(pool: &PgPool) -> Result<(), sqlx::Error> {
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
async fn database_table_creation_function_users(pool: &PgPool) -> Result<(), sqlx::Error> {
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
async fn singleton_database_instance_launcher() -> PgPool {
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = singleton_database_instance_launcher().await;
    database_table_creation_function_users(&pool)
        .await
        .expect("Error in database_table_creation_function_users");

    database_table_creation_function_token(&pool)
        .await
        .expect("Error in database_table_creation_function_token");
    database_check_for_outtime_tokens(&pool)
        .await
        .expect("Error in database_check_for_outtime_tokens");
    let addr = "127.0.0.1:8080";
    println!("Server running on http://{}", addr);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone())) // <- Inject pool
            .route("/", web::get().to(greet))
            .route("/api/register", web::post().to(register_handler))
            .route("/api/login", web::post().to(login_handler))
            .route("/api/logout", web::post().to(logout_handler))
    })
    .bind(addr)?
    .run()
    .await
}
