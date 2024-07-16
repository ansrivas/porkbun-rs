use crate::client::HTTPClient;
use crate::errors::PorkbunnError;
use crate::{make_json_request, make_request};
use reqwest::header::HeaderValue;
use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseListDomains {
    pub status: String,
    pub domains: Vec<Domain>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseCreateRecord {
    pub id: u64,
    pub status: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseDeleteRecord {
    pub status: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseListDnsRecords {
    pub status: String,
    pub records: Vec<Record>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Record {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub content: String,
    pub ttl: String,
    pub prio: Option<String>,
    pub notes: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Domain {
    pub auto_renew: String,
    pub create_date: String,
    pub domain: String,
    pub expire_date: String,
    pub not_local: u32,
    pub status: Option<String>,
    pub tld: String,
    pub whois_privacy: u32,
    pub security_lock: u32,
}

/// The `PorkbunnClient` struct represents a client for interacting with the Porkbun API.
pub struct PorkbunnClient {
    http_client: HTTPClient,
    api_key: String,
    api_secret: String,
}

impl PorkbunnClient {
    /// Creates a new `PorkbunnClient` instance.
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base URL of the Porkbun API.
    /// * `version` - The version of the Porkbun API.
    /// * `api_key` - The API key for authentication.
    /// * `api_secret` - The API secret for authentication.
    ///
    /// # Returns
    ///
    /// A new `PorkbunnClient` instance.
    fn inner_client(
        base_url: &str,
        version: &str,
        api_key: &str,
        api_secret: &str,
    ) -> PorkbunnClient {
        // Create headers with content-type set to application/json
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "content-type",
            HeaderValue::from_str("application/json").unwrap(),
        );

        // Build the HTTP client with default headers
        let client = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .build()
            .unwrap();

        PorkbunnClient {
            http_client: HTTPClient::new(base_url, client, version),
            api_key: api_key.to_string(),
            api_secret: api_secret.to_string(),
        }
    }

    /// Creates a new `PorkbunnClient` instance.
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base URL of the Porkbun API.
    /// * `version` - The version of the Porkbun API.
    /// * `api_key` - The API key for authentication.
    /// * `api_secret` - The API secret for authentication.
    ///
    /// # Returns
    ///
    /// A new `PorkbunnClient` instance.
    pub fn new(base_url: &str, version: &str, api_key: &str, api_secret: &str) -> PorkbunnClient {
        PorkbunnClient::inner_client(base_url, version, api_key, api_secret)
    }

    /// Retrieves a list of DNS records for a given name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the DNS records to retrieve.
    ///
    /// # Returns
    ///
    /// A `Result` containing the response data or an error of type `PorkbunnError`.
    pub async fn list_dns_records(
        &self,
        name: &str,
    ) -> Result<ResponseListDnsRecords, PorkbunnError> {
        let url = &format!("dns/retrieve/{}", name);
        make_request!(self, reqwest::Method::POST, url)
    }

    /// Creates a new DNS record.
    ///
    /// # Arguments
    ///
    /// * `domain` - The domain for which to create the DNS record.
    /// * `name` - The name of the DNS record.
    /// * `record_type` - The type of the DNS record.
    /// * `ip_address` - The IP address associated with the DNS record.
    /// * `ttl` - The time-to-live value for the DNS record.
    ///
    /// # Returns
    ///
    /// A `Result` containing the response data or an error of type `PorkbunnError`.
    pub async fn create_dns_record(
        &self,
        domain: &str,
        name: &str,
        record_type: &str,
        ip_address: &str,
        ttl: u32,
    ) -> Result<ResponseCreateRecord, PorkbunnError> {
        let url = &format!("dns/create/{}", domain);
        let payload = &serde_json::json!({
            "apikey": self.api_key,
            "secretapikey": self.api_secret,
            "name": name,
            "type": record_type,
            "content": ip_address,
            "ttl": ttl,
        });
        make_json_request!(self, reqwest::Method::POST, url, payload)
    }

    /// Deletes a DNS record.
    ///
    /// # Arguments
    ///
    /// * `domain` - The domain for which to delete the DNS record.
    /// * `id` - The ID of the DNS record to delete.
    ///
    /// # Returns
    ///
    /// A `Result` containing the response data or an error of type `PorkbunnError`.
    pub async fn delete_dns_record(
        &self,
        domain: &str,
        id: u64,
    ) -> Result<ResponseDeleteRecord, PorkbunnError> {
        let url = &format!("dns/delete/{}/{}", domain, id);
        make_request!(self, reqwest::Method::POST, url)
    }

    /// Retrieves a list of all domains.
    ///
    /// # Returns
    ///
    /// A `Result` containing the response data or an error of type `PorkbunnError`.
    pub async fn list_domains(&self) -> Result<ResponseListDomains, PorkbunnError> {
        let url = "domain/listAll";
        make_request!(self, reqwest::Method::POST, url)
    }
}
