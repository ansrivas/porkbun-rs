use crate::errors::PorkbunnError;
use percent_encoding::{percent_encode, NON_ALPHANUMERIC};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct HTTPClient {
    client: reqwest::Client,
    base_url: reqwest::Url,
    version: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct APIError {
    pub more_info: Option<String>,
    pub status: Option<i32>,
    pub message: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct APIResponse {
    pub errors: Option<Vec<APIError>>,
    pub message: Option<String>,
}

/// Percent encode an incoming parameter
pub(crate) fn encode_param(param: &str) -> String {
    percent_encode(param.as_bytes(), NON_ALPHANUMERIC).to_string()
}

/// Make a http request by providing a json-body
#[macro_export]
macro_rules! make_json_request {
    ($sel:ident, $method:path, $url:expr, $body:ident) => {{
        use crate::{client::APIResponse, errors::PorkbunnError};
        use reqwest;
        use tracing::error;

        let response: reqwest::Response = $sel
            .http_client
            .inner($method, $url)?
            .json($body)
            .send()
            .await?;
        let status_code = &response.status().as_u16();

        if !(*status_code >= 200 && *status_code < 300) {
            error!("status_code = {}", status_code);
            error!("url queried = {}", $url);
            let api_response: APIResponse = response.json().await?;
            return Err(PorkbunnError::APIResponseError {
                errors: api_response.errors.unwrap(),
                message: api_response.message.unwrap(),
            });
        }
        let ret: Result<reqwest::Response, PorkbunnError> = Ok(response);
        ret
    }};
}

/// Make a http request without json body.
#[macro_export]
macro_rules! make_request {
    ($sel:ident, $method:path, $url:expr) => {{
        use reqwest;
        use serde_json::json;

        let body = json!({
            "apikey": $sel.api_key,
            "secretapikey": $sel.api_secret,
        });
        let response: reqwest::Response = $sel.http_client.inner($method, $url)?.json(&body).send().await?;
        use crate::client::APIResponse;

        let status_code = &response.status().as_u16();
        tracing::debug!("Received http status code: {}", status_code);
        tracing::debug!("Sending requests to url: {}", $url);

        if !(*status_code >= 200 && *status_code < 300) {
            let api_response: serde_json::Value = response.json().await?;
            tracing::debug!("Received api response: {:#?}", api_response);
            let api_response: APIResponse = serde_json::from_value(api_response).unwrap();
            return Err(PorkbunnError::APIResponseError {
                errors: api_response.errors.unwrap(),
                message: api_response.message.unwrap(),
            });
        }


            let ret: Result<reqwest::Response, PorkbunnError> = Ok(response);
            ret



    }};
}

impl HTTPClient {
    pub fn new<S, T>(base_url: S, client: reqwest::Client, version: T) -> HTTPClient
    where
        S: Into<String>,
        T: Into<String>,
    {
        let parsed_url =
            reqwest::Url::parse(&base_url.into()).expect("Failed to parse the base_url");

        let ver = format!("{}/", version.into().replace("/", ""));
        tracing::debug!("API Version is {}", &ver);
        HTTPClient {
            base_url: parsed_url,
            client,
            version: ver,
        }
    }

    pub(crate) fn inner(
        &self,
        method: reqwest::Method,
        query_url: &str,
    ) -> Result<reqwest::RequestBuilder, PorkbunnError> {
        let qurl = query_url.trim_start_matches('/');
        let url = self.base_url.join(&self.version)?.join(qurl)?;
        tracing::debug!("URL is {:?}", &url);

        // dbg!(&url);
        let request_with_url_and_header: Result<reqwest::RequestBuilder, PorkbunnError> =
            match method {
                reqwest::Method::GET => Ok(self.client.get(url)),
                reqwest::Method::PUT => Ok(self.client.put(url)),
                reqwest::Method::POST => Ok(self.client.post(url)),
                reqwest::Method::DELETE => Ok(self.client.delete(url)),
                _ => return Err(PorkbunnError::UnsupportedMethod),
            };
        request_with_url_and_header
    }
}
