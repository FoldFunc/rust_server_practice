use actix_web::cookie::{Cookie, SameSite};
use actix_web::{HttpRequest, HttpResponse, Responder, web};
use cookie::time;
use rand::{Rng, random};
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
    email: String,
    password: String,
}
pub async fn login_handler(
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
    } else {
        HttpResponse::BadRequest().body("No auth cookie found")
    }
}

#[derive(serde::Deserialize)]
pub struct ChangeDataStruct {
    pub name: String,
}

pub async fn change_price_handler(
    req: HttpRequest,
    db_pool: web::Data<PgPool>,
    change_price_data: web::Json<ChangeDataStruct>,
) -> impl Responder {
    // Optional: token check
    if let Some(cookie) = req.cookie("auth_root") {
        let token_value = cookie.value();
        // TODO: validate token_value if needed
    } else {
        return HttpResponse::Unauthorized().body("Missing authentication cookie");
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
    let adjustment = current_price % 10;
    let new_price = rng.gen_range(current_price - adjustment..=current_price + adjustment);

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
