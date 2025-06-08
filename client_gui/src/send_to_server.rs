use reqwest::Client;
use reqwest::cookie::Jar;
use serde::Serialize;
use std::fs;
use std::sync::Arc;
#[derive(Serialize)]
struct Login {
    email: String,
    password: String,
}
#[derive(Serialize)]
struct Register {
    email: String,
    password: String,
}
#[tokio::main]
pub async fn send_login_data(
    email: String,
    password: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let cookie_jar = Arc::new(Jar::default());
    let client = build_client_with_cookies(cookie_jar.clone())?;

    let login_data = Login { email, password };
    let res = client
        .post("http://localhost:8080/api/login")
        .json(&login_data)
        .send()
        .await?;
    save_cookie(&res)?;

    println!("Status: {}", res.status());
    println!("Server response: {}", res.text().await?);
    Ok(())
}
#[tokio::main]
pub async fn send_register_data(
    email: String,
    password: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let cookie_jar = Arc::new(Jar::default());
    let client = build_client_with_cookies(cookie_jar.clone())?;
    let register_data = Register { email, password };
    let res = client
        .post("http://localhost:8080/api/register")
        .json(&register_data)
        .send()
        .await?;
    println!("Status: {}", res.status());
    println!("Server response: {}", res.text().await?);
    Ok(())
}

pub fn build_client_with_cookies(cookie_jar: Arc<Jar>) -> Result<Client, reqwest::Error> {
    Client::builder().cookie_provider(cookie_jar).build()
}
pub fn save_cookie(res: &reqwest::Response) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(cookie_header) = res.headers().get("set-cookie") {
        let cookie_str = cookie_header.to_str().unwrap_or("");
        let _ = fs::write("src/cookie/cookie.txt", cookie_str);
        println!("Cookie saved: {}", cookie_str);
    } else {
        println!("No cookie recived");
    }
    Ok(())
}
