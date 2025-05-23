use reqwest::Client;
use serde::Serialize;

#[derive(Serialize)]
struct ToSent {
    name: String,
    secret_key: String,
}
// Make a function that will fetch all of the crypto names and update the "name" in the secret_key so the call to the server endpoint will be possibe
// Also need to change server endpoint to accept the "secret" key not a cookie.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let secret_key = ToSent {
        name: "Solana".to_string(),
        secret_key: "secret_no_tell".to_string(),
    };
    loop {
        let res = client
            .post("http://localhost:8080/api/root/changeprice")
            .json(&secret_key)
            .send()
            .await?;
        println!("Status: {}", res.status());
        println!("Response: {}", res.text().await?);
    }
}
