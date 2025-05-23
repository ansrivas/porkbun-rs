use clap::{CommandFactory, Parser, Subcommand, ValueEnum};

use crate::{porkbunn_client, serde_ext::SerdeExt};
use clap_complete::{Generator, Shell, generate};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    // If provided, outputs the completion file for given shell
    #[arg(long = "generate", value_enum)]
    generator: Option<Shell>,

    #[command(subcommand)]
    command: Option<Commands>,

    #[clap(
        long,
        short = 'b',
        env = "BASE_URL",
        default_value = "https://api.porkbun.com/api/json/"
    )]
    base_url: String,

    #[clap(long, short = 'v', env = "BASE_URL_VERSION", default_value = "v3")]
    url_version: String,

    /// API key for the porkbun API
    #[clap(long, short = 'a', env = "API_KEY")]
    api_key: Option<String>,

    /// Secret key for the porkbun API
    #[clap(long, short = 's', env = "SECRET_KEY")]
    secret_key: Option<String>,
}

#[derive(Debug, PartialEq, ValueEnum, Clone)]
enum RecordType {
    A,
    Mx,
    Cname,
    Alias,
    Txt,
    Ns,
    Aaaa,
    Srv,
    Tlsa,
    Caa,
    Https,
    Svcb,
}

impl std::fmt::Display for RecordType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = match self {
            RecordType::A => "A".to_string(),
            RecordType::Mx => "MX".to_string(),
            RecordType::Cname => "CNAME".to_string(),
            RecordType::Alias => "ALIAS".to_string(),
            RecordType::Txt => "TXT".to_string(),
            RecordType::Ns => "NS".to_string(),
            RecordType::Aaaa => "AAAA".to_string(),
            RecordType::Srv => "SRV".to_string(),
            RecordType::Tlsa => "TLSA".to_string(),
            RecordType::Caa => "CAA".to_string(),
            RecordType::Https => "HTTPS".to_string(),
            RecordType::Svcb => "SVCB".to_string(),
        };
        write!(f, "{}", v)
    }
}

fn print_completions<G: Generator>(gene: G, cmd: &mut clap::Command) {
    generate(
        gene,
        cmd,
        cmd.get_name().to_string(),
        &mut std::io::stdout(),
    );
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new DNS record for a given domain
    CreateRecord {
        /// Time to live
        #[arg(short, long, value_name = "TTL")]
        ttl: u32,

        /// Record type
        #[arg(short, long, value_name = "RECORD_TYPE", value_enum)]
        record_type: RecordType,

        /// Name for e.g. `index`` if the expected dns record is for index.example.com and example.com is the domain
        #[arg(short, long, value_name = "NAME")]
        name: String,

        /// Domain for which we are setting the record for e.g. example.com
        #[arg(short, long, value_name = "DOMAIN")]
        domain: String,

        /// IP address for the DNS record
        #[arg(short, long, value_name = "IP_ADDRESS")]
        ip_address: String,

        /// Delete the record if it already exists
        #[arg(short, long, value_name = "DELETE_EXISTING")]
        delete_existing: bool,
    },

    /// Delete a DNS record for a given domain
    DeleteRecord {
        /// Domain
        #[arg(short, long, value_name = "DOMAIN")]
        domain: String,

        /// ID of the record
        #[arg(short, long, value_name = "ID")]
        id: u64,

        /// Skip confirmation prompt
        #[arg(short, long)]
        skip_confirm: bool,
    },

    /// List all domains associated with the account
    ListDomains,

    /// List all records for a given domain
    ListRecords {
        /// Domain
        #[arg(short, long, value_name = "DOMAIN")]
        domain: String,
    },
}

/// Prompts the user for input and returns a boolean value based on the user's response.
///
/// This function displays the provided message to the user and waits for their input. If the user
/// enters "y" (case-insensitive), the function returns `true`. If the user enters "n" (case-insensitive),
/// the function returns `false`. If the user enters any other value, the function prints an error
/// message and recursively calls itself to prompt the user again.
///
/// # Arguments
///
/// * `msg` - The message to display to the user when prompting for input.
///
/// # Returns
///
/// A boolean value indicating the user's response.
fn ensure_input(msg: &str) -> bool {
    println!("{}", msg);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input = input.trim().to_string();
    if input.to_lowercase() == "y" {
        return true;
    } else if input.to_lowercase() == "n" {
        return false;
    } else {
        println!("Invalid input, please enter y or n");
        ensure_input(msg)
    }
}

/// Runs the CLI application.
///
/// This function is the entry point for the CLI application. It parses the command-line arguments,
/// sets up the logging, and then executes the appropriate command based on the user's input.
///
/// # Errors
///
/// This function can return any error that may occur during the execution of the CLI commands.
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    // ensure that .env files are also supported
    dotenvy::dotenv().ok();

    let cli = Cli::parse();

    if cli.debug > 0 {
        unsafe {
            std::env::set_var("RUST_LOG", "debug");
        }
    } else {
        unsafe {
            std::env::set_var("RUST_LOG", "info");
        }
    }
    tracing_subscriber::fmt::init();

    if let Some(generator) = cli.generator {
        print_completions(generator, &mut Cli::command());
        return Ok(());
    }

    assert!(cli.api_key.is_some(), "API_KEY is not set");
    assert!(cli.secret_key.is_some(), "SECRET_KEY is not set");

    let client = porkbunn_client::PorkbunnClient::new(
        &cli.base_url,
        &cli.url_version,
        &cli.api_key.unwrap(),
        &cli.secret_key.unwrap(),
    );
    match &cli.command {
        Some(Commands::CreateRecord {
            ttl,
            record_type,
            domain,
            name,
            ip_address,
            delete_existing,
        }) => {
            tracing::debug!(
                "Registering {}.{} with ttl {} and record type {}",
                name,
                domain,
                ttl,
                record_type.to_string()
            );
            if *delete_existing {
                let records = client.list_dns_records(name).await?;
                for record in records.records {
                    if record.name == *name && record.type_field == record_type.to_string() {
                        if let Ok(id) = record.id.parse::<u64>() {
                            tracing::info!("Deleting existing record with id {}", id);
                            client.delete_dns_record(domain, id).await?;
                        }
                    }
                }
            }

            client
                .create_dns_record(
                    domain,
                    name,
                    &record_type.to_string().to_uppercase(),
                    ip_address,
                    *ttl,
                )
                .await?
                .pretty_print();
        }
        Some(Commands::DeleteRecord {
            domain,
            id,
            skip_confirm,
        }) => {
            tracing::debug!("Deleting {} with id {}", domain, id);
            if *skip_confirm {
                client.delete_dns_record(domain, *id).await?.pretty_print();
                return Ok(());
            }

            if ensure_input("Are you sure you want to delete this record? (y/n)") {
                client.delete_dns_record(domain, *id).await?.pretty_print();
            } else {
                println!("Record not deleted");
                return Ok(());
            }
        }
        Some(Commands::ListDomains) => {
            client.list_domains().await?.pretty_print();
        }
        Some(Commands::ListRecords { domain }) => {
            client.list_dns_records(domain).await?.pretty_print();
        }
        None => {
            // print help and exit
            let _ = Cli::command().print_help();
        }
    };
    Ok(())
}
