use reqwest::Client;
use reqwest::cookie::Jar;
use serde::Serialize;

use std::fs;
use std::io::{self, Write};
use std::sync::Arc;
#[derive(Serialize)]
struct Register {
    email: String,
    password: String,
}

#[derive(Serialize)]
struct Login {
    email: String,
    password: String,
}
#[derive(Serialize)]
struct ChangePrice {
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let has_account = input("Do you have an account? (yes/no): ").to_lowercase();

    if has_account == "yes" {
        login_flow().await?;
    } else {
        register_flow().await?;
    }

    Ok(())
}

async fn login_flow() -> Result<(), Box<dyn std::error::Error>> {
    println!("Please log in:");
    let email = input("Enter email: ");
    let password = input("Enter password: ");
    let login_data = Login { email, password };

    let cookie_jar = Arc::new(Jar::default());
    let client = build_client_with_cookies(cookie_jar.clone())?;

    let res = client
        .post("http://localhost:8080/api/login")
        .json(&login_data)
        .send()
        .await?;

    save_cookie(&res)?;

    println!("Status: {}", res.status());
    println!("Server response: {}", res.text().await?);

    handle_commands(client, cookie_jar).await?;
    Ok(())
}

async fn register_flow() -> Result<(), Box<dyn std::error::Error>> {
    let email = input("Enter email: ");
    let password = input("Enter password: ");
    let reg_data = Register { email, password };

    let client = Client::new();
    let res = client
        .post("http://localhost:8080/api/register")
        .json(&reg_data)
        .send()
        .await?;

    println!("Status: {}", res.status());
    println!("Response: {}", res.text().await?);
    login_flow().await?;
    Ok(())
}

async fn handle_commands(
    client: Client,
    cookie_jar: Arc<Jar>,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let cmd = input("What to do (help, logout): ");
        match cmd.trim() {
            "help" => println!("Available commands: help, logout"),
            "logout" => {
                let logout_client = build_client_with_cookies(cookie_jar.clone())?;
                let res = logout_client
                    .post("http://localhost:8080/api/logout")
                    .send()
                    .await?;

                println!("Logout status: {}", res.status());
                println!("Response: {}", res.text().await?);
                break;
            }
            "change price" => {
                let change_price = build_client_with_cookies(cookie_jar.clone())?;
                let change_data = ChangePrice {
                    name: "bitcoin".to_string(),
                };
                let res = change_price
                    .post("http://localhost:8080/api/root/changeprice")
                    .json(&change_data)
                    .send()
                    .await?;
                println!("Status: {}", res.status());
                println!("Response: {}", res.text().await?);
                break;
            }
            _ => println!("Unknown command."),
        }
    }
    Ok(())
}

fn build_client_with_cookies(cookie_jar: Arc<Jar>) -> Result<Client, reqwest::Error> {
    Client::builder().cookie_provider(cookie_jar).build()
}

fn save_cookie(res: &reqwest::Response) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(cookie_header) = res.headers().get("set-cookie") {
        let cookie_str = cookie_header.to_str().unwrap_or("");
        fs::write("cookie.txt", cookie_str)?;
        println!("Cookie saved: {}", cookie_str);
    } else {
        println!("No cookie received");
    }
    Ok(())
}

fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading input");
    input.trim().to_string()
}
