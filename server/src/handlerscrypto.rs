use actix_web::{HttpRequest, HttpResponse, Responder, web};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};

#[derive(Deserialize, Debug, Serialize)]
pub struct CryptoName {
    name: String,
}

#[derive(Serialize, Debug)]
pub struct CryptoPrice {
    price: i32,
}

#[derive(Serialize, Deserialize, Debug)] // FIXED: Added Deserialize
pub struct FetchSpecificStock {
    name: String,
}

pub async fn fetchstocknames(req: HttpRequest, db_pool: web::Data<PgPool>) -> impl Responder {
    let cookie = match req.cookie("auth") {
        Some(c) => c,
        None => return HttpResponse::Unauthorized().body("Missing cookie"),
    };

    let token_value = cookie.value();

    let _owner_row = match sqlx::query("SELECT owner FROM token WHERE token=$1")
        .bind(token_value)
        .fetch_optional(db_pool.get_ref())
        .await
    {
        Ok(Some(row)) => row,
        Ok(None) => return HttpResponse::Unauthorized().body("Invalid cookie"),
        Err(e) => {
            eprintln!("DB error: {}", e);
            return HttpResponse::InternalServerError().body(format!("Database error: {}", e));
        }
    };

    let result = sqlx::query("SELECT name FROM crypto")
        .fetch_all(db_pool.get_ref())
        .await;

    match result {
        Ok(rows) => {
            let names: Vec<CryptoName> = rows
                .into_iter()
                .map(|row| CryptoName {
                    name: row.try_get("name").unwrap_or_default(),
                })
                .collect();

            HttpResponse::Ok().json(names)
        }
        Err(e) => {
            eprintln!("DB error: {}", e);
            HttpResponse::InternalServerError().body("Failed to fetch names")
        }
    }
}

pub async fn fetchstockprices(req: HttpRequest, db_pool: web::Data<PgPool>) -> impl Responder {
    let cookie = match req.cookie("auth") {
        Some(c) => c,
        None => return HttpResponse::Unauthorized().body("Missing cookie"),
    };

    let token_value = cookie.value();

    let _owner_row = match sqlx::query("SELECT owner FROM token WHERE token=$1")
        .bind(token_value)
        .fetch_optional(db_pool.get_ref())
        .await
    {
        Ok(Some(row)) => row,
        Ok(None) => return HttpResponse::Unauthorized().body("Invalid cookie"),
        Err(e) => {
            eprintln!("DB error: {}", e);
            return HttpResponse::InternalServerError().body(format!("Database error: {}", e));
        }
    };

    let result = sqlx::query("SELECT price FROM crypto")
        .fetch_all(db_pool.get_ref())
        .await;

    match result {
        Ok(rows) => {
            let prices: Vec<CryptoPrice> = rows
                .into_iter()
                .map(|row| CryptoPrice {
                    price: row.try_get("price").unwrap_or_default(),
                })
                .collect();
            println!("{:?}", prices);
            HttpResponse::Ok().json(prices)
        }
        Err(e) => {
            eprintln!("DB error: {}", e);
            HttpResponse::InternalServerError().body("Failed to fetch prices")
        }
    }
}

pub async fn fetchstockspecific(
    fetch_stock_specific_data: web::Json<FetchSpecificStock>,
    req: HttpRequest,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let cookie = match req.cookie("auth") {
        Some(c) => c,
        None => return HttpResponse::Unauthorized().body("Missing cookie"),
    };

    let token_value = cookie.value();

    let _owner_row = match sqlx::query("SELECT owner FROM token WHERE token=$1")
        .bind(token_value)
        .fetch_optional(db_pool.get_ref())
        .await
    {
        Ok(Some(row)) => row,
        Ok(None) => return HttpResponse::Unauthorized().body("Invalid cookie"),
        Err(e) => {
            eprintln!("DB error: {}", e);
            return HttpResponse::InternalServerError().body(format!("Database error: {}", e));
        }
    };

    let pattern = format!("%{}%", &fetch_stock_specific_data.name); // Match anywhere in the name
    let result = sqlx::query("SELECT name FROM crypto WHERE name LIKE $1")
        .bind(pattern)
        .fetch_all(db_pool.get_ref())
        .await;

    match result {
        Ok(rows) => {
            let names: Vec<CryptoName> = rows
                .into_iter()
                .map(|row| CryptoName {
                    name: row.try_get("name").unwrap_or_default(),
                })
                .collect();
            println!("{:?}", names);
            HttpResponse::Ok().json(names)
        }
        Err(e) => {
            eprintln!("DB error: {}", e);
            HttpResponse::InternalServerError().body("Failed to fetch specific data")
        }
    }
}
