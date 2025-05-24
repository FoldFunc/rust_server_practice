use actix_web::{HttpRequest, HttpResponse, Responder, web};
use serde::Deserialize;
use sqlx::{PgPool, Row};

#[derive(Deserialize, Debug)]
pub struct BuyCryptoData {
    portfolioname: String,
    portfoliopassword: String,
    crypto_to_buy: String,
    amount: i32,
}

pub async fn buycrypto(
    data: web::Json<BuyCryptoData>,
    db_pool: web::Data<PgPool>,
    req: HttpRequest,
) -> impl Responder {
    let cookie = match req.cookie("auth") {
        Some(c) => c,
        None => return HttpResponse::Unauthorized().body("Missing cookie"),
    };
    let token_value = cookie.value();

    // Validate token
    let _owner = match sqlx::query("SELECT owner FROM token WHERE token = $1")
        .bind(token_value)
        .fetch_optional(db_pool.get_ref())
        .await
    {
        Ok(Some(row)) => row.try_get::<String, _>("owner").unwrap_or_default(),
        Ok(None) => return HttpResponse::Unauthorized().body("Invalid cookie"),
        Err(e) => return HttpResponse::InternalServerError().body(format!("DB error: {}", e)),
    };

    // Validate portfolio password
    let portfolio_row = sqlx::query("SELECT password, money FROM portfolios WHERE name = $1")
        .bind(&data.portfolioname)
        .fetch_optional(db_pool.get_ref())
        .await;

    let (stored_password, mut money): (String, i32) = match portfolio_row {
        Ok(Some(row)) => (
            row.try_get("password").unwrap_or_default(),
            row.try_get("money").unwrap_or_default(),
        ),
        Ok(None) => return HttpResponse::BadRequest().body("Portfolio not found"),
        Err(e) => return HttpResponse::InternalServerError().body(format!("DB error: {}", e)),
    };

    if stored_password != data.portfoliopassword {
        return HttpResponse::BadRequest().body("Invalid portfolio password");
    }

    // Get crypto price
    let price_row = sqlx::query("SELECT price FROM crypto WHERE name = $1")
        .bind(&data.crypto_to_buy)
        .fetch_optional(db_pool.get_ref())
        .await;

    let price = match price_row {
        Ok(Some(row)) => row.try_get::<i32, _>("price").unwrap_or(0),
        Ok(None) => return HttpResponse::BadRequest().body("Invalid crypto name"),
        Err(e) => return HttpResponse::InternalServerError().body(format!("DB error: {}", e)),
    };

    let total_cost = price * data.amount;

    if money < total_cost {
        return HttpResponse::BadRequest().body("Not enough money");
    }

    money -= total_cost;

    // Simulate "buying" the crypto — store the transaction and deduct money
    let tx_result = sqlx::query("UPDATE portfolios SET money = $1 WHERE name = $2")
        .bind(money)
        .bind(&data.portfolioname)
        .execute(db_pool.get_ref())
        .await;

    if let Err(e) = tx_result {
        return HttpResponse::InternalServerError()
            .body(format!("Failed to update portfolio: {}", e));
    }

    // TODO: Add crypto to holdings table, or log transaction

    HttpResponse::Ok().body(format!(
        "Successfully bought {} of {} for {}",
        data.amount, data.crypto_to_buy, total_cost
    ))
}
