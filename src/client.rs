use crate::errors::PorkbunnError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct HTTPClient {
    client: reqwest::Client,
    base_url: reqwest::Url,
    version: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct APIError {
    pub status: Option<String>,
    pub message: Option<String>,
}

/// Make a http request by providing a json-body
#[macro_export]
/// Macro for making a JSON request using the specified HTTP method, URL, and request body.
///
/// # Arguments
///
/// * `$sel`: The selector for the HTTP client.
/// * `$method`: The HTTP method to use for the request.
/// * `$url`: The URL to send the request to.
/// * `$body`: The request body.
///
/// # Returns
///
/// Returns a `Result` containing the JSON response if the request is successful, or an `APIError` if the request fails.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate porkbun_rs;
/// # use reqwest::Method;
/// # use porkbun_rs::client::APIError;
/// # use porkbun_rs::errors::PorkbunnError;
/// # use porkbun_rs::client::PorkbunClient;
/// # use serde_json::Value;
/// # use tracing::error;
/// # #[cfg(feature = "debug")]
/// # use serde_json::from_value;
/// # #[cfg(not(feature = "debug"))]
/// # use serde_json::from_value;
/// #
/// # struct MyHttpClient;
/// # impl MyHttpClient {
/// #     fn inner(&self, method: Method, url: &str) -> Result<reqwest::RequestBuilder, reqwest::Error> {
/// #         unimplemented!()
/// #     }
/// # }
/// #
/// # impl PorkbunClient for MyHttpClient {
/// #     fn http_client(&self) -> &MyHttpClient {
/// #         self
/// #     }
/// # }
/// #
/// # fn main() -> Result<(), PorkbunnError> {
/// #     let sel = MyHttpClient;
/// #     let method = Method::GET;
/// #     let url = "https://example.com";
/// #     let body = serde_json::json!({});
/// #
/// let response = make_json_request!(sel, method, url, body);
/// match response {
///     Ok(json) => {
///         // Handle successful response
///     }
///     Err(error) => {
///         // Handle API error
///     }
/// }
/// #
/// #     Ok(())
/// # }
/// ```
///
/// Note: This macro requires the `reqwest`, `tracing`, `serde_json`, `APIError`, and `PorkbunnError` dependencies to be in scope.
macro_rules! make_json_request {
    ($sel:ident, $method:path, $url:expr, $body:ident) => {{
        use reqwest;
        use tracing::error;
        use $crate::{client::APIError, errors::PorkbunnError};

        tracing::debug!(
            "make_json_request: method = {}, url = {} body = {:?}",
            stringify!($method),
            $url,
            $body
        );
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
            let api_response: serde_json::Value = response.json().await?;
            tracing::debug!("Received api response: {:#?}", api_response);
            let api_response: APIError = serde_json::from_value(api_response).unwrap();
            return Err(PorkbunnError::APIResponseError {
                status: api_response.status.unwrap_or_default(),
                message: api_response.message.unwrap_or_default(),
            });
        }

        #[cfg(feature = "debug")]
        {
            let res: serde_json::Value = response.json().await?;
            tracing::debug!("Response {:?}", res);
            Ok(serde_json::from_value(res)?)
        }

        #[cfg(not(feature = "debug"))]
        Ok(response.json().await?)
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
        use $crate::client::APIError;

        let status_code = &response.status().as_u16();
        tracing::debug!("Received http status code: {}", status_code);
        tracing::debug!("Sending requests to url: {}", $url);

        if !(*status_code >= 200 && *status_code < 300) {
            let api_response: serde_json::Value = response.json().await?;
            tracing::debug!("Received api response: {:#?}", api_response);
            let api_response: APIError = serde_json::from_value(api_response).unwrap();
            return Err(PorkbunnError::APIResponseError {
                status: api_response.status.unwrap_or_default(),
                message: api_response.message.unwrap_or_default(),
            });
        }

        #[cfg(feature = "debug")]
        {
            let res: serde_json::Value = response.json().await?;
            tracing::debug!("Response {:?}", res);
            Ok(serde_json::from_value(res)?)
        }

        #[cfg(not(feature = "debug"))]
        Ok(response.json().await?)
    }};
}

/// Represents an HTTP client for making requests to a specific base URL and API version.
///
/// The `HTTPClient` struct provides a convenient way to make HTTP requests to a specific base URL and API version.
/// It wraps the `reqwest` library to provide a higher-level interface for making requests.
///
/// The `new` function creates a new `HTTPClient` instance by parsing the provided base URL and API version.
/// The `inner` function is used to create a `reqwest::RequestBuilder` for a specific HTTP method and URL path.
///
/// This implementation is an internal detail of the crate and is not intended to be used directly by end-users.
impl HTTPClient {
    pub fn new<S, T>(base_url: S, client: reqwest::Client, version: T) -> HTTPClient
    where
        S: Into<String>,
        T: Into<String>,
    {
        let parsed_url =
            reqwest::Url::parse(&base_url.into()).expect("Failed to parse the base_url");

        let ver = format!("{}/", version.into().replace('/', ""));
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
