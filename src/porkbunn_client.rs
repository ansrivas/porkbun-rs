use std::vec;

use crate::client::encode_param;
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
    pub status: String,
    pub id: String,
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

pub struct PorkbunnClient {
    http_client: HTTPClient,
    api_key: String,
    api_secret: String,
}

impl PorkbunnClient {
    fn inner_client(
        base_url: &str,
        version: &str,
        api_key: &str,
        api_secret: &str,
    ) -> PorkbunnClient {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "content-type",
            HeaderValue::from_str("application/json").unwrap(),
        );

        // We are unwrapping here only because we want it to fail early
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

    pub fn new(base_url: &str, version: &str, api_key: &str, api_secret: &str) -> PorkbunnClient {
        PorkbunnClient::inner_client(base_url, version, api_key, api_secret)
    }

    pub async fn list_dns_records(
        &self,
        name: &str,
    ) -> Result<ResponseListDnsRecords, PorkbunnError> {
        let url = &format!("dns/retrieve/{}", name);
        let response = make_request!(self, reqwest::Method::POST, url)?;

        #[cfg(feature = "debug")]
        {
            let res: serde_json::Value = response.json().await?;
            tracing::debug!("Response {:?}", res);
            Ok(serde_json::from_value(res)?)
        }

        #[cfg(not(feature = "debug"))]
        Ok(response.json().await?)
    }

    pub async fn create_dns_record(
        &self,
        name: &str,
        record_type: &str,
        ip_address: &str,
        ttl: u32,
    ) -> Result<ResponseCreateRecord, PorkbunnError> {
        let url = &format!("dns/create/{}", encode_param(name));
        let payload = &serde_json::json!({
            "apikey": self.api_key,
            "secretapikey": self.api_secret,
            "name": name,
            "type": record_type,
            "content": ip_address,
            "ttl": ttl,
        });
        let response = make_json_request!(self, reqwest::Method::POST, url, payload)?;
        Ok(response.json().await?)
    }

    pub async fn delete_dns_record(&self) -> Result<(), PorkbunnError> {
        Ok(())
    }

    pub async fn list_domains(&self) -> Result<ResponseListDomains, PorkbunnError> {
        let url = "domain/listAll";
        let response = make_request!(self, reqwest::Method::POST, url)?;
        Ok(response.json().await?)

        // make_request!(self, reqwest::Method::POST, url)

        // #[cfg(feature = "debug")]
        // {
        //     let res: serde_json::Value = response.json().await?;
        //     tracing::debug!("Response {:?}", res);
        //     Ok(serde_json::from_value(res)?)
        // }

        // #[cfg(not(feature = "debug"))]
        // Ok(response.json().await?)
    }
}
