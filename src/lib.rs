#[macro_use]
extern crate log;
extern crate hyper;
extern crate rustc_serialize;
extern crate url;
extern crate serde;
extern crate serde_json;

use hyper::client::{Client, RequestBuilder};
use hyper::method::Method;
use hyper::header::{ContentType, Authorization, Basic};
use std::io::Read;

mod builder;
pub use builder::*;
mod errors;
pub use errors::*;
mod rep;
pub use rep::*;

pub type Result<T> = std::result::Result<T, Error>;

pub enum Credentials {
    Basic(String, String), // todo: OAuth
}

/// https://docs.atlassian.com/jira/REST/latest/
pub struct Jira<'a> {
    host: String,
    credentials: Credentials,
    client: &'a Client,
}

impl<'a> Jira<'a> {
    pub fn new<H>(host: H, credentials: Credentials, client: &'a Client) -> Jira<'a>
        where H: Into<String>
    {
        Jira {
            host: host.into(),
            credentials: credentials,
            client: client,
        }
    }

    /// https://docs.atlassian.com/jira/REST/latest/#api/2/search
    pub fn search(&self, opts: &SearchOptions) -> Result<SearchResults> {
        let mut path = vec!["/search".to_owned()];
        if let Some(q) = opts.serialize() {
            path.push(q);
        }
        let body = try!(self.get(path.join("?").as_ref()));
        let parsed = try!(serde_json::from_str(&body));
        Ok(parsed)
    }

    // https://docs.atlassian.com/jira/REST/latest/#api/2/issue
    pub fn issue(&self, id: &str) -> Result<String> {
        self.get(format!("/issue/{}", id).as_ref())
    }

    fn get(&self, endpoint: &str) -> Result<String> {
        self.request(Method::Get, endpoint)
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

    fn request(&self, method: Method, endpoint: &str) -> Result<String> {
        let req = self.authenticate(method, endpoint)
            .header(ContentType::json());
        let mut res = try!(req.send());
        let mut buf = String::new();
        try!(res.read_to_string(&mut buf));
        debug!("{:?}", buf);
        Ok(buf)
    }
}


#[test]
fn it_works() {}
