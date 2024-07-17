// #![deny(missing_docs)]

use porkbun_rs::cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    cli::run().await?;
    Ok(())
}
