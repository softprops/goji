//! Goji provides an interface for Jira's REST api

#[macro_use]
extern crate log;
extern crate hyper;
extern crate url;
extern crate serde;
extern crate serde_json;

use hyper::client::{Client, RequestBuilder};
use hyper::method::Method;
use hyper::header::{ContentType, Authorization, Basic};
use serde::{Deserialize, Serialize};
use std::io::Read;

mod transitions;
pub use transitions::*;
mod issues;
pub use issues::*;
mod search;
pub use search::*;
mod builder;
pub use builder::*;
mod errors;
pub use errors::*;
mod rep;
pub use rep::*;

pub type Result<T> = std::result::Result<T, Error>;

/// Types of authentication credentials
pub enum Credentials {
    /// username and password credentials
    Basic(String, String), // todo: OAuth
}

/// Entrypoint into client interface
/// https://docs.atlassian.com/jira/REST/latest/
pub struct Jira<'a> {
    host: String,
    credentials: Credentials,
    client: &'a Client,
}

impl<'a> Jira<'a> {
    /// creates a new instance of a jira client
    pub fn new<H>(host: H, credentials: Credentials, client: &'a Client) -> Jira<'a>
        where H: Into<String>
    {
        Jira {
            host: host.into(),
            credentials: credentials,
            client: client,
        }
    }

    /// return transitions interface
    pub fn transitions<K>(&self, key: K) -> Transitions
        where K: Into<String>
    {
        Transitions::new(self, key)
    }

    /// return search interface
    pub fn search(&self) -> Search {
        Search::new(self)
    }


    // return issues interface
    pub fn issues(&self) -> Issues {
        Issues::new(self)
    }

    fn post<D, S>(&self, endpoint: &str, body: S) -> Result<D>
        where D: Deserialize,
              S: Serialize
    {
        let data = try!(serde_json::to_string::<S>(&body));
        self.request::<D>(Method::Post, endpoint, Some(data.as_bytes()))
    }

    fn get<D>(&self, endpoint: &str) -> Result<D>
        where D: Deserialize
    {
        self.request::<D>(Method::Get, endpoint, None)
    }

    fn authenticate(&self, method: Method, uri: &str) -> RequestBuilder {
        let url = format!("{}/rest/api/latest{}", self.host, uri);
        debug!("url -> {:?}", url);
        match self.credentials {
            Credentials::Basic(ref user, ref pass) => {
                self.client
                    .request(method, &url)
                    .header(Authorization(Basic {
                        username: user.to_owned(),
                        password: Some(pass.to_owned()),
                    }))
            }
        }
    }

    fn request<D>(&self, method: Method, endpoint: &str, body: Option<&'a [u8]>) -> Result<D>
        where D: Deserialize
    {
        let builder = self.authenticate(method, endpoint)
            .header(ContentType::json());

        let mut res = try!(match body {
            Some(ref bod) => builder.body(*bod).send(),
            _ => builder.send(),
        });
        let mut body = String::new();
        try!(res.read_to_string(&mut body));
        debug!("status {:?} body '{:?}'", res.status, body);
        if res.status.is_client_error() {
            Err(Error::Fault {
                code: res.status,
                errors: try!(serde_json::from_str::<Errors>(&body)),
            })
        } else {
            Ok(try!(serde_json::from_str::<D>(&body)))
        }
    }
}

#[test]
fn it_works() {}
