use reqwest::cookie::Jar;
use reqwest::{Client, Response};
use serde_json;
use std::error::Error;
use std::fs;
use std::sync::Arc;
use std::time::SystemTime;
const BASE_URL: &str = "http://localhost:8080";

pub async fn send_login_data(email: String, password: String) -> Result<(), Box<dyn Error>> {
    println!("send_login_data function called");
    let res = send_credentials("/api/login", email, password).await?;
    println!("Login response: {}", res);

    if res.trim() != "Login successful" {
        return Err("Login failed".into());
    }

    Ok(())
}

pub async fn send_register_data(email: String, password: String) -> Result<(), Box<dyn Error>> {
    println!("send_register_data function called");
    let res = send_credentials("/api/register", email, password).await?;
    println!("Register response: {}", res);
    if res.trim() != "Register successful" {
        return Err("Registration failed".into());
    }
    Ok(())
}

async fn send_credentials(
    endpoint: &str,
    email: String,
    password: String,
) -> Result<String, Box<dyn Error>> {
    let client = build_client_with_cookies().await?;

    let res = client
        .post(&format!("{}{}", BASE_URL, endpoint))
        .json(&serde_json::json!({ "email": email, "password": password }))
        .send()
        .await?;

    save_cookie(&res).await?; // <- this line ensures the cookie is saved

    let body = res.text().await?;
    Ok(body)
}

/// Builds HTTP client with cookie handling
async fn build_client_with_cookies() -> Result<Client, reqwest::Error> {
    let cookie_jar = Arc::new(Jar::default());
    Client::builder().cookie_provider(cookie_jar).build()
}

/// Saves session cookie to a file
async fn save_cookie(res: &Response) -> Result<(), Box<dyn Error>> {
    println!("Saving cookies...");
    if let Some(cookie_header) = res.headers().get("set-cookie") {
        let cookie_str = cookie_header.to_str()?;
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs();
        fs::write(
            "src/cookie/cookie.txt",
            format!("{} | {}\n", timestamp, cookie_str),
        )?;
        println!("Cookie saved: {}", cookie_str);
    } else {
        println!("No cookie received.");
    }
    Ok(())
}
