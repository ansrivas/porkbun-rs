// #![deny(missing_docs)]

mod cli;
mod client;
mod errors;
mod porkbunn_client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    cli::run().await;
    Ok(())

    // let domain = "ansrivasdev.de".to_string();
    // let uri = format!("https://api.porkbun.com/api/json/v3/dns/create/{}", domain.url_encode());
    // let body = r#"{
    // "apikey": "pk1_5f62bdb90783c6892e95030805d495b6cb939015a9cd3f443d2218edac28bc20",
    // "secretapikey": "sk1_27a69f55ce93fd02c985bd5ffdd832aff83ed63a39072cffef8562d4f2df8e73",
    // "name": "portal2",
    // "type": "A",
    // "content": "65.21.181.212",
    // "ttl": "120"
    // }"#;
    // let mut resp = isahc::post(uri, body).unwrap();
    // println!("Status: {}", resp.status());
    // println!("body: {:?}", resp.body());
    // println!("text: {:?}", resp.text());

    // let domain = "ansrivasdev.de".to_string();
    // let uri = format!("https://api.porkbun.com/api/json/v3/dns/delete/{}/{}", domain.url_encode(), "413620139");
    // let body = r#"{
    // "apikey": "pk1_5f62bdb90783c6892e95030805d495b6cb939015a9cd3f443d2218edac28bc20",
    // "secretapikey": "sk1_27a69f55ce93fd02c985bd5ffdd832aff83ed63a39072cffef8562d4f2df8e73"
    // }"#;
    // let mut resp = isahc::post(uri, body).unwrap();
    // println!("Status: {}", resp.status());
    // println!("body: {:?}", resp.body());
    // println!("text: {:?}", resp.text());
}
