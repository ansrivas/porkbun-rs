use std::path::PathBuf;

use clap::{CommandFactory, Parser, Subcommand, ValueEnum};

use crate::{porkbunn_client, serde_ext::SerdeExt};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

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
    api_key: String,

    #[clap(long, short = 's', env = "SECRET_KEY")]
    secret_key: String,
}

#[derive(Debug, PartialEq, ValueEnum, Clone)]
enum RecordType {
    A,
    MX,
    CNAME,
    ALIAS,
    TXT,
    NS,
    AAAA,
    SRV,
    TLSA,
    CAA,
    HTTPS,
    SVCB,
}

impl RecordType {
    fn to_string(&self) -> String {
        match self {
            RecordType::A => "A".to_string(),
            RecordType::MX => "MX".to_string(),
            RecordType::CNAME => "CNAME".to_string(),
            RecordType::ALIAS => "ALIAS".to_string(),
            RecordType::TXT => "TXT".to_string(),
            RecordType::NS => "NS".to_string(),
            RecordType::AAAA => "AAAA".to_string(),
            RecordType::SRV => "SRV".to_string(),
            RecordType::TLSA => "TLSA".to_string(),
            RecordType::CAA => "CAA".to_string(),
            RecordType::HTTPS => "HTTPS".to_string(),
            RecordType::SVCB => "SVCB".to_string(),
        }
    }

    fn from_string(s: &str) -> Option<RecordType> {
        match s {
            "A" => Some(RecordType::A),
            "MX" => Some(RecordType::MX),
            "CNAME" => Some(RecordType::CNAME),
            "ALIAS" => Some(RecordType::ALIAS),
            "TXT" => Some(RecordType::TXT),
            "NS" => Some(RecordType::NS),
            "AAAA" => Some(RecordType::AAAA),
            "SRV" => Some(RecordType::SRV),
            "TLSA" => Some(RecordType::TLSA),
            "CAA" => Some(RecordType::CAA),
            "HTTPS" => Some(RecordType::HTTPS),
            "SVCB" => Some(RecordType::SVCB),
            _ => None,
        }
    }
}

#[derive(Subcommand)]
enum Commands {
    CreateRecord {
        /// Time to live
        #[arg(short, long, value_name = "TTL")]
        ttl: u32,

        /// Record type
        #[arg(short, long, value_name = "RECORD_TYPE", value_enum)]
        record_type: RecordType,

        /// Name for e.g. `index`` if the expected dns record is for index.example.com and example.com is the domain
        #[arg(short, long, value_name = "DOMAIN")]
        domain: String,

        /// IP address for the DNS record
        #[arg(short, long, value_name = "IP_ADDRESS")]
        ip_address: String,
    },
    DeleteRecord {
        /// Domain
        #[arg(short, long, value_name = "DOMAIN")]
        domain: String,

        /// ID
        #[arg(short, long, value_name = "ID")]
        id: String,
    },

    /// List all domains
    ListDomains,

    /// List all records for a given domain
    ListRecords {
        /// Domain
        #[arg(short, long, value_name = "DOMAIN")]
        domain: String,
    },
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();

    // You can check the value provided by positional arguments, or option arguments
    if let Some(name) = cli.name.as_deref() {
        println!("Value for name: {name}");
    }

    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    // match cli.debug {
    //     0 => println!("Debug mode is off"),
    //     1 => println!("Debug mode is kind of on"),
    //     2 => println!("Debug mode is on"),
    //     _ => println!("Don't be crazy"),
    // }

    let client = porkbunn_client::PorkbunnClient::new(
        &cli.base_url,
        &cli.url_version,
        &cli.api_key,
        &cli.secret_key,
    );

    match &cli.command {
        Some(Commands::CreateRecord {
            ttl,
            record_type,
            domain,
            ip_address,
        }) => {
            tracing::debug!(
                "Registering {} with ttl {} and record type {}",
                domain,
                ttl,
                record_type.to_string()
            );
            client
                .create_dns_record(
                    domain,
                    &record_type.to_string().to_uppercase(),
                    &ip_address,
                    *ttl,
                )
                .await?
                .pretty_print();
        }
        Some(Commands::DeleteRecord { domain, id }) => {
            println!("Deleting {} with id {}", domain, id);
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
