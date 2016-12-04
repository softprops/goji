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
use hyper::status::StatusCode;
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
        match res.status {
            StatusCode::Unauthorized => {
                // returns unparsable html
                Err(Error::Unauthorized)
            },
            client_err if client_err.is_client_error() => {
                Err(Error::Fault {
                    code: res.status,
                    errors: try!(serde_json::from_str::<Errors>(&body)),
                })
            },
            _ => {
                Ok(try!(serde_json::from_str::<D>(&body)))
            }
        }
    }
}

pub struct SearchIter<'a> {
    jira: &'a Jira<'a>,
    jql: String,
    results: SearchResults
}

impl<'a> SearchIter<'a> {
    pub fn new<J>(jql: J, options: &SearchOptions, jira: &'a Jira<'a>) -> Result<SearchIter<'a>> where J: Into<String> {
        let query = jql.into();
        let results = try!(jira.search().list(query.clone(), options));
        Ok(SearchIter {
            jira: jira,
            jql: query,
            results: results
        })
    }

    fn more(&self) -> bool {
        (self.results.start_at + self.results.issues.len() as u64) < self.results.total
    }
}

impl <'a> Iterator for SearchIter<'a> {
    type Item = Issue;
    fn next(&mut self) -> Option<Issue> {
        self.results.issues.pop().or_else(||
            if self.more() {
                println!("fetchig more...");
                match self.jira.search().list(
                    self.jql.clone(),
                    &SearchOptions::builder()
                        .max_results(self.results.max_results)
                        .start_at(self.results.start_at + self.results.max_results)
                        .build()) {
                            Ok(new_results) => {
                                self.results = new_results;
                                self.results.issues.pop()
                            },
                            _ => None
                        }
            } else {
                None
            }
        )
    }
}

#[test]
fn it_works() {}
