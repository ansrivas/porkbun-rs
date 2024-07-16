use clap::{CommandFactory, Parser, Subcommand, ValueEnum};

use crate::{porkbunn_client, serde_ext::SerdeExt};
use clap_complete::{generate, Generator, Shell};

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

    /// Project where we need to commit file to.
    #[clap(long, short = 'a', env = "API_KEY")]
    api_key: Option<String>,

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

fn print_completions<G: Generator>(gen: G, cmd: &mut clap::Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut std::io::stdout());
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
    },

    /// Delete a DNS record for a given domain
    DeleteRecord {
        /// Domain
        #[arg(short, long, value_name = "DOMAIN")]
        domain: String,

        /// ID of the record
        #[arg(short, long, value_name = "ID")]
        id: u64,
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

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if cli.debug > 0 {
        std::env::set_var("RUST_LOG", "debug");
    } else {
        std::env::set_var("RUST_LOG", "info");
    }
    tracing_subscriber::fmt::init();

    if let Some(generator) = cli.generator {
        print_completions(generator, &mut Cli::command());
        return Ok(());
    }

    assert!(cli.api_key.is_some(), "API_KEY is not set");
    assert!(cli.secret_key.is_some(), "API_KEY is not set");

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
        }) => {
            tracing::debug!(
                "Registering {}.{} with ttl {} and record type {}",
                name,
                domain,
                ttl,
                record_type.to_string()
            );
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
        Some(Commands::DeleteRecord { domain, id }) => {
            tracing::debug!("Deleting {} with id {}", domain, id);
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
