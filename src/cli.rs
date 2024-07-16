use clap::{CommandFactory, Parser, Subcommand, ValueEnum};

use crate::{porkbunn_client, serde_ext::SerdeExt};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
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
        #[arg(short, long, value_name = "NAME")]
        name: String,

        /// Domain for which we are setting the record for e.g. example.com
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

        /// ID of the record
        #[arg(short, long, value_name = "ID")]
        id: u64,
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
            client.delete_dns_record(domain, *id).await?.pretty_print();
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
