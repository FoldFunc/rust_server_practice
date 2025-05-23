use actix_web::cookie::{Cookie, SameSite};
use actix_web::{HttpRequest, HttpResponse, Responder, web};
use cookie::time;
use rand::Rng;
use serde::Deserialize;
use sqlx::PgPool;
use sqlx::Row;
use uuid::Uuid;
#[derive(Deserialize)]
pub struct RegisterDataStruct {
    email: String,
    password: String,
}

pub async fn register_handler(
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
pub struct LoginDataStruct {
    pub email: String,
    pub password: String,
}

pub async fn login_handler(
    login_data: web::Json<LoginDataStruct>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    // Validate credentials
    let user_exists = sqlx::query("SELECT 1 FROM users WHERE email = $1 AND password = $2")
        .bind(&login_data.email)
        .bind(&login_data.password)
        .fetch_optional(db_pool.get_ref())
        .await;

    if let Ok(Some(_)) = user_exists {
        // Check if token already exists for user
        let existing_token = sqlx::query("SELECT token FROM token WHERE owner = $1")
            .bind(&login_data.email)
            .fetch_optional(db_pool.get_ref())
            .await;

        match existing_token {
            Ok(Some(_)) => {
                // Token exists, check if admin
                let is_admin = sqlx::query("SELECT 1 FROM whitelist WHERE email = $1")
                    .bind(&login_data.email)
                    .fetch_optional(db_pool.get_ref())
                    .await;

                let cookie_name = if is_admin.is_ok() && is_admin.unwrap().is_some() {
                    "auth_root"
                } else {
                    "auth"
                };

                let token = Uuid::new_v4().to_string();
                let cookie = Cookie::build(cookie_name, &token)
                    .path("/")
                    .http_only(true)
                    .secure(false)
                    .same_site(SameSite::Lax)
                    .max_age(time::Duration::days(1))
                    .finish();
                // Insert token in DB
                let insert_result = sqlx::query("INSERT INTO token (token, owner) VALUES ($1, $2)")
                    .bind(&token)
                    .bind(&login_data.email)
                    .execute(db_pool.get_ref())
                    .await;

                if insert_result.is_err() {
                    return HttpResponse::InternalServerError().body("Failed to insert token");
                }

                return HttpResponse::Ok()
                    .cookie(cookie)
                    .body(if cookie_name == "auth_root" {
                        "Admin login successful"
                    } else {
                        "Login successful"
                    });
            }
            Ok(None) => {
                // No token exists, create new
                let token = Uuid::new_v4().to_string();

                // Check admin status
                let is_admin = sqlx::query("SELECT 1 FROM whitelist WHERE email = $1")
                    .bind(&login_data.email)
                    .fetch_optional(db_pool.get_ref())
                    .await;

                let cookie_name = if is_admin.is_ok() && is_admin.unwrap().is_some() {
                    "auth_root"
                } else {
                    "auth"
                };

                // Insert token in DB
                let insert_result = sqlx::query("INSERT INTO token (token, owner) VALUES ($1, $2)")
                    .bind(&token)
                    .bind(&login_data.email)
                    .execute(db_pool.get_ref())
                    .await;

                if insert_result.is_err() {
                    return HttpResponse::InternalServerError().body("Failed to insert token");
                }

                let cookie = Cookie::build(cookie_name, &token)
                    .path("/")
                    .http_only(true)
                    .secure(false)
                    .same_site(SameSite::Lax)
                    .max_age(time::Duration::days(1))
                    .finish();

                return HttpResponse::Ok()
                    .cookie(cookie)
                    .body(if cookie_name == "auth_root" {
                        "Admin login successful"
                    } else {
                        "Login successful"
                    });
            }
            Err(_) => {
                return HttpResponse::InternalServerError().body("Database error checking token");
            }
        }
    } else if let Ok(None) = user_exists {
        HttpResponse::Unauthorized().body("Invalid email or password")
    } else {
        HttpResponse::InternalServerError().body("Database error during login")
    }
}
pub async fn logout_handler(req: HttpRequest, db_pool: web::Data<PgPool>) -> impl Responder {
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
    } else if let Some(cookie) = req.cookie("auth_root") {
        let token_value = cookie.value();

        // Delete the token row from DB by token value
        let result = sqlx::query("DELETE FROM token WHERE token = $1")
            .bind(token_value)
            .execute(db_pool.get_ref())
            .await;

        match result {
            Ok(_) => {
                // Remove cookie on client side by setting a cookie with max_age = 0
                let expired_cookie = Cookie::build("auth_root", "")
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
        return HttpResponse::BadRequest().body("No auth cookie found");
    }
}

#[derive(serde::Deserialize)]
pub struct ChangeDataStruct {
    pub name: String,
    pub secret_key: String,
}

pub async fn change_price_handler(
    db_pool: web::Data<PgPool>,
    change_price_data: web::Json<ChangeDataStruct>,
) -> impl Responder {
    if change_price_data.secret_key != "secret_no_tell" {
        eprintln!("Invalid token");
        return HttpResponse::InternalServerError().body("Invalid token");
    }
    // Get current price
    let row = match sqlx::query("SELECT price FROM crypto WHERE name = $1")
        .bind(&change_price_data.name)
        .fetch_one(db_pool.get_ref())
        .await
    {
        Ok(row) => row,
        Err(e) => {
            eprintln!("DB error: {}", e);
            return HttpResponse::InternalServerError().body("Error fetching price");
        }
    };

    let current_price: i32 = row.get("price");

    // Generate new price around ±10
    let mut rng = rand::thread_rng();
    let adjustment = rng.gen_range(1..=10); // always at least 1
    let direction = if rng.gen_bool(0.5) { 1 } else { -1 }; // up or down
    let new_price = (current_price as i32 + direction * adjustment) as i32;
    println!("New price: {}", new_price);

    // Update new price
    let update_result = sqlx::query("UPDATE crypto SET price = $1 WHERE name = $2")
        .bind(new_price)
        .bind(&change_price_data.name)
        .execute(db_pool.get_ref())
        .await;

    match update_result {
        Ok(_) => HttpResponse::Ok().body("Price changed"),
        Err(e) => {
            eprintln!("Error updating price: {}", e);
            HttpResponse::InternalServerError().body("Failed to update price")
        }
    }
}

pub async fn create_a_root(req: HttpRequest, db_pool: web::Data<PgPool>) -> impl Responder {
    // 1. Get the "auth" cookie
    println!("req: {:?}", req);
    let cookie = match req.cookie("auth") {
        Some(c) => c,
        None => return HttpResponse::Unauthorized().body("Missing cookie"),
    };

    let token_value = cookie.value();

    // 2. Check if token is valid and get associated email
    let row = match sqlx::query("SELECT owner FROM token WHERE token = $1")
        .bind(token_value)
        .fetch_optional(db_pool.get_ref())
        .await
    {
        Ok(Some(row)) => row,
        Ok(None) => return HttpResponse::Unauthorized().body("Invalid token"),
        Err(e) => {
            eprintln!("DB error (token lookup): {}", e);
            return HttpResponse::InternalServerError().body("Database error");
        }
    };

    let email: String = row.get("owner");

    // 3. Check if user is on the whitelist
    let is_whitelisted = match sqlx::query("SELECT 1 FROM whitelist WHERE email = $1")
        .bind(&email)
        .fetch_optional(db_pool.get_ref())
        .await
    {
        Ok(Some(_)) => true,
        Ok(None) => false,
        Err(e) => {
            eprintln!("DB error (whitelist check): {}", e);
            return HttpResponse::InternalServerError().body("Database error");
        }
    };

    if !is_whitelisted {
        return HttpResponse::Unauthorized().body("Not on whitelist");
    }

    // 4. Set a new cookie "auth_root"
    let auth_root_cookie = Cookie::build("auth_root", token_value.to_string())
        .path("/")
        .http_only(true)
        .secure(false)
        .finish();

    // 5. Return response with new cookie
    HttpResponse::Ok()
        .cookie(auth_root_cookie)
        .body("Root access granted")
}
#[derive(Deserialize)]
pub struct CreateCryptoStruct {
    pub name: String,
    pub price: i32,
}

pub async fn create_crypto(
    req: HttpRequest,
    db_pool: web::Data<PgPool>,
    create_crypto_data: web::Json<CreateCryptoStruct>,
) -> impl Responder {
    println!("req: {:?}", req);
    // Step 1: Validate admin token
    let cookie = match req.cookie("auth_root") {
        Some(c) => c,
        None => return HttpResponse::Unauthorized().body("Missing or invalid cookie"),
    };

    let token_value = cookie.value();

    // Step 2: Lookup owner from token
    let row = match sqlx::query("SELECT owner FROM token WHERE token = $1")
        .bind(token_value)
        .fetch_optional(db_pool.get_ref())
        .await
    {
        Ok(Some(row)) => row,
        Ok(None) => return HttpResponse::Unauthorized().body("Invalid token"),
        Err(e) => {
            eprintln!("DB error (token lookup): {}", e);
            return HttpResponse::InternalServerError().body(format!("Database error: {}", e));
        }
    };

    let owner: String = row.get("owner");

    // Step 3: Insert new crypto
    let result = sqlx::query("INSERT INTO crypto (name, creator, price) VALUES ($1, $2, $3)")
        .bind(&create_crypto_data.name)
        .bind(&owner)
        .bind(create_crypto_data.price)
        .execute(db_pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Created().body("Crypto created!"),
        Err(e) => {
            eprintln!("DB error (insert crypto): {}", e);
            HttpResponse::InternalServerError().body("Error inserting crypto")
        }
    }
}
