// #![deny(missing_docs)]

mod cli;
mod client;
mod errors;
mod porkbunn_client;
mod serde_ext;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    cli::run().await?;
    Ok(())
}
