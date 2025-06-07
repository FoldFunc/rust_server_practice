use reqwest::Client;
use reqwest::cookie::Jar;
use serde::{Deserialize, Serialize};
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

#[derive(Serialize)]
struct CreateCrypto {
    name: String,
    price: i32,
}
#[derive(Serialize)]
struct RemoveCrypto {
    name: String,
}
#[derive(Serialize)]
struct AddPortfolio {
    password: String,
    name: String,
}
#[derive(Serialize)]
struct DeletePortfolio {
    password: String,
    name: String,
}
#[derive(Deserialize, Debug)]
struct FetchCryptoNames {
    name: String,
}
#[derive(Deserialize, Debug)]
struct FetchCryptoPrises {
    price: i32,
}
#[derive(Deserialize, Debug, Serialize)]
struct FetchCryptoSpecifc {
    name: String,
}
#[derive(Deserialize, Debug, Serialize)]
struct BuyCrypto {
    portfolioname: String,
    portfoliopassword: String,
    crypto_to_buy: String,
    amount: i32,
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to root managment");
    let cookie_jar = Arc::new(Jar::default());
    let client = build_client_with_cookies(cookie_jar.clone())?;

    let has_account = input("Do you have an account? (yes/no): ").to_lowercase();

    if has_account == "yes" {
        login_flow(&client).await?;
    } else {
        register_flow(&client).await?;
        login_flow(&client).await?;
    }

    handle_commands(&client).await?;

    Ok(())
}

async fn login_flow(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    println!("Please log in:");
    let email = input("Enter email: ");
    let password = input("Enter password: ");
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

async fn register_flow(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let email = input("Enter email: ");
    let password = input("Enter password: ");
    let reg_data = Register { email, password };

    let res = client
        .post("http://localhost:8080/api/register")
        .json(&reg_data)
        .send()
        .await?;

    println!("Status: {}", res.status());
    println!("Response: {}", res.text().await?);
    Ok(())
}

async fn handle_commands(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let cmd = input(
            "What to do (help, logout, change price, create crypto, get root, remove crypto): ",
        );
        match cmd.trim() {
            "help" => println!("Available commands: help, logout"),
            "logout" => {
                let res = client
                    .post("http://localhost:8080/api/logout")
                    .send()
                    .await?;

                println!("Logout status: {}", res.status());
                println!("Response: {}", res.text().await?);
                break;
            }
            "change price" => {
                let change_data = ChangePrice {
                    name: "bitcoin".to_string(),
                };
                let res = client
                    .post("http://localhost:8080/api/root/changeprice")
                    .json(&change_data)
                    .send()
                    .await?;
                println!("Status: {}", res.status());
                println!("Response: {}", res.text().await?);
            }
            "get root" => {
                let res = client
                    .post("http://localhost:8080/api/getroot")
                    .send()
                    .await?;
                println!("Status: {}", res.status());
                println!("Response: {}", res.text().await?);
            }
            "create crypto" => {
                let crypto_name = input("Enter crypto name: ");
                let crypto_price = loop {
                    let input_str = input("Enter crypto starting price: ");
                    match input_str.trim().parse::<i32>() {
                        Ok(price) => break price,
                        Err(_) => println!("Invalid number, try again."),
                    }
                };

                let crypto = CreateCrypto {
                    name: crypto_name,
                    price: crypto_price,
                };

                let res = client
                    .post("http://localhost:8080/api/root/addcrypto")
                    .json(&crypto)
                    .send()
                    .await?;

                println!("Status: {}", res.status());
                let body = res.text().await?;
                println!("Response: {}", body);
            }
            "remove crypto" => {
                let crypto_name = input("Enter crypto name: ");
                let crypto = RemoveCrypto { name: crypto_name };
                let res = client
                    .post("http://localhost:8080/api/root/removecrypto")
                    .json(&crypto)
                    .send()
                    .await?;

                println!("Status: {}", res.status());
                println!("Status: {}", res.text().await?);
            }
            "add portfolio" => {
                let portfolio_name = input("Enter portfolio name: ");
                let portfolio_password = input("Enter portfolio password: ");
                let portfolio = AddPortfolio {
                    password: portfolio_password,
                    name: portfolio_name,
                };
                let res = client
                    .post("http://localhost:8080/api/addportfolio")
                    .json(&portfolio)
                    .send()
                    .await?;
                println!("Status: {}", res.status());
                println!("Rsponse : {}", res.text().await?);
            }
            "delete portfolio" => {
                let sure = input("Are you sure you want to delete the portfolio(yes, no): ")
                    .to_lowercase();
                if sure == "no" {
                    break;
                }
                let portfolio_name = input("Enter portfolio name for verification: ");
                let portfolio_password = input("Enter portfolio password for verification: ");
                let portfolio = DeletePortfolio {
                    password: portfolio_password,
                    name: portfolio_name,
                };
                let res = client
                    .post("http://localhost:8080/api/deleteportfolio")
                    .json(&portfolio)
                    .send()
                    .await?;
                println!("Status: {}", res.status());
                println!("Rsponse : {}", res.text().await?);
            }
            "fetch crypto names" => {
                let res = client
                    .post("http://localhost:8080/api/fetch/cryptonames")
                    .send()
                    .await?;

                println!("Status: {}", res.status());

                let crypto_names: Vec<FetchCryptoNames> = res.json().await?;
                println!("JSON received: {:?}", crypto_names);
            }
            "fetch crypto prices" => {
                let res = client
                    .post("http://localhost:8080/api/fetch/cryptoprices")
                    .send()
                    .await?;

                println!("Status: {}", res.status());

                let crypto_names: Vec<FetchCryptoPrises> = res.json().await?;
                println!("JSON received: {:?}", crypto_names);
            }
            "fetch crypto specific" => {
                let name = input("What crypto name to look for: ");
                let specific = FetchCryptoSpecifc { name: name };
                let res = client
                    .post("http://localhost:8080/api/fetch/cryptospecific")
                    .json(&specific)
                    .send()
                    .await?;

                println!("Status: {}", res.status());

                let crypto_names: Vec<FetchCryptoSpecifc> = res.json().await?;
                println!("JSON received: {:?}", crypto_names);
            }
            "buy crypto" => {
                let crypto_name =
                    input("Give the name of the crypto you won't to buy(full name): ");
                let amount = loop {
                    let input_str = input("How many of this stocks would you like: ");
                    match input_str.trim().parse::<i32>() {
                        Ok(price) => break price,
                        Err(_) => println!("Invalid number try again"),
                    }
                };
                let portfolio_name = input("To which portfolio you want to add it?: ");
                let portfolio_password = input("Give me the password for this portfolio: ");

                let crypto_buy = BuyCrypto {
                    portfolioname: portfolio_name,
                    portfoliopassword: portfolio_password,
                    crypto_to_buy: crypto_name,
                    amount: amount,
                };

                let res = client
                    .post("http://localhost:8080/api/crypto/buycrypto")
                    .json(&crypto_buy)
                    .send()
                    .await?;
                println!("Status: {}", res.status());
                let body = res.text().await?;
                println!("Response: {}", body);
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
