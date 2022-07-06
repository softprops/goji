//! Gouqi provides an interface for Jira's REST api

extern crate reqwest;
extern crate serde;
extern crate tracing;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate url;

use std::io::Read;
use tracing::debug;

use reqwest::header::CONTENT_TYPE;
use reqwest::{
    blocking::{Client, RequestBuilder},
    Method, StatusCode,
};
use serde::de::DeserializeOwned;
use serde::Serialize;
use url::Url;

pub mod attachments;
mod builder;
pub mod components;
mod errors;
pub mod issues;
mod rep;
mod search;
mod transitions;
mod versions;

pub use crate::attachments::*;
pub use crate::builder::*;
pub use crate::components::*;
pub use crate::errors::*;
pub use crate::issues::*;
pub use crate::rep::*;
pub use crate::search::Search;
pub use crate::transitions::*;
pub mod boards;
pub mod resolution;
pub use crate::boards::*;
pub mod sprints;
pub use crate::sprints::*;
pub use crate::versions::*;

#[derive(Deserialize, Debug)]
pub struct EmptyResponse;

pub type Result<T> = std::result::Result<T, Error>;

/// Types of authentication credentials
///
/// # Notes
///
/// - Personal Access Token are used with [`Credentials::Basic`] scheme as a password replacement and *not* as a [`Credentials::Bearer`]
///   like the [API documentation sugest](https://developer.atlassian.com/server/jira/platform/rest-apis/#authentication-and-authorization).
#[derive(Clone, Debug)]
pub enum Credentials {
    /// Use no authentication
    Anonymous,
    /// Username and password credentials (Personal Access Token count as a password)
    Basic(String, String),
    /// Authentification via bearer token
    Bearer(String),
    // TODO: Add OAuth
}

impl Credentials {
    fn apply(&self, request: RequestBuilder) -> RequestBuilder {
        match self {
            Credentials::Anonymous => request,
            Credentials::Basic(ref user, ref pass) => {
                request.basic_auth(user.to_owned(), Some(pass.to_owned()))
            }
            Credentials::Bearer(ref token) => request.bearer_auth(token.to_owned()),
        }
    }
}

/// Entrypoint into client interface
/// <https://docs.atlassian.com/jira/REST/latest/>
#[derive(Clone, Debug)]
pub struct Jira {
    host: Url,
    credentials: Credentials,
    client: Client,
}

impl Jira {
    /// Creates a new instance of a jira client
    pub fn new<H>(host: H, credentials: Credentials) -> Result<Jira>
    where
        H: Into<String>,
    {
        match Url::parse(&host.into()) {
            Ok(host) => Ok(Jira {
                host,
                client: Client::new(),
                credentials,
            }),
            Err(error) => Err(Error::from(error)),
        }
    }

    /// Creates a new instance of a jira client using a specified reqwest client
    pub fn from_client<H>(host: H, credentials: Credentials, client: Client) -> Result<Jira>
    where
        H: Into<String>,
    {
        match Url::parse(&host.into()) {
            Ok(host) => Ok(Jira {
                host,
                client,
                credentials,
            }),
            Err(error) => Err(Error::from(error)),
        }
    }

    /// Return transitions interface
    pub fn transitions<K>(&self, key: K) -> Transitions
    where
        K: Into<String>,
    {
        Transitions::new(self, key)
    }

    /// Return search interface
    #[tracing::instrument]
    pub fn search(&self) -> Search {
        Search::new(self)
    }

    // Return issues interface
    #[tracing::instrument]
    pub fn issues(&self) -> Issues {
        Issues::new(self)
    }

    // Return attachments interface
    pub fn attachments(&self) -> Attachments {
        Attachments::new(self)
    }

    // Return components interface
    pub fn components(&self) -> Components {
        Components::new(self)
    }

    // Return boards interface
    #[tracing::instrument]
    pub fn boards(&self) -> Boards {
        Boards::new(self)
    }

    // Return boards interface
    #[tracing::instrument]
    pub fn sprints(&self) -> Sprints {
        Sprints::new(self)
    }

    #[tracing::instrument]
    pub fn versions(&self) -> Versions {
        Versions::new(self)
    }

    pub fn session(&self) -> Result<Session> {
        self.get("auth", "/session")
    }

    #[tracing::instrument]
    fn delete<D>(&self, api_name: &str, endpoint: &str) -> Result<D>
    where
        D: DeserializeOwned,
    {
        self.request::<D>(Method::DELETE, api_name, endpoint, None)
    }

    #[tracing::instrument]
    fn get<D>(&self, api_name: &str, endpoint: &str) -> Result<D>
    where
        D: DeserializeOwned,
    {
        self.request::<D>(Method::GET, api_name, endpoint, None)
    }

    fn post<D, S>(&self, api_name: &str, endpoint: &str, body: S) -> Result<D>
    where
        D: DeserializeOwned,
        S: Serialize,
    {
        let data = serde_json::to_string::<S>(&body)?;
        debug!("Json POST request: {}", data);
        self.request::<D>(Method::POST, api_name, endpoint, Some(data.into_bytes()))
    }

    fn put<D, S>(&self, api_name: &str, endpoint: &str, body: S) -> Result<D>
    where
        D: DeserializeOwned,
        S: Serialize,
    {
        let data = serde_json::to_string::<S>(&body)?;
        debug!("Json request: {}", data);
        self.request::<D>(Method::PUT, api_name, endpoint, Some(data.into_bytes()))
    }

    #[tracing::instrument]
    fn request<D>(
        &self,
        method: Method,
        api_name: &str,
        endpoint: &str,
        body: Option<Vec<u8>>,
    ) -> Result<D>
    where
        D: DeserializeOwned,
    {
        let url = self
            .host
            .join(&format!("rest/{}/latest{}", api_name, endpoint))?;
        debug!("url -> {:?}", url);

        let mut req = self
            .client
            .request(method, url)
            .header(CONTENT_TYPE, "application/json");

        req = self.credentials.apply(req);

        if let Some(body) = body {
            req = req.body(body);
        }
        debug!("req '{:?}'", req);

        let mut res = req.send()?;

        let mut body = String::new();
        res.read_to_string(&mut body)?;
        debug!("status {:?} body '{:?}'", res.status(), body);
        match res.status() {
            StatusCode::UNAUTHORIZED => Err(Error::Unauthorized),
            StatusCode::METHOD_NOT_ALLOWED => Err(Error::MethodNotAllowed),
            StatusCode::NOT_FOUND => Err(Error::NotFound),
            client_err if client_err.is_client_error() => Err(Error::Fault {
                code: res.status(),
                errors: serde_json::from_str::<Errors>(&body)?,
            }),
            _ => {
                let data = if body.is_empty() { "null" } else { &body };
                Ok(serde_json::from_str::<D>(data)?)
            }
        }
    }
}
