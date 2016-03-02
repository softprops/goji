#[macro_use]
extern crate log;
extern crate hyper;
extern crate rustc_serialize;
extern crate url;
extern crate serde;
extern crate serde_json;

pub mod errors;
pub use errors::Error;
pub mod rep;
pub use rep::SearchResults;
use hyper::client::Client;
use hyper::method::Method;
use hyper::header::{ContentType, Authorization, Basic};
use std::io::Read;

pub mod builder;

pub use builder::SearchOptions;

pub type Result<T> = std::result::Result<T, Error>;

/// https://docs.atlassian.com/jira/REST/latest/
pub struct Jira {
    host: String,
    username: String,
    password: String,
}

impl Jira {
    pub fn new<U,P, H>(host: H, username: U, password: P) -> Jira where U: Into<String>, P: Into<String>, H: Into<String> {
        Jira {
            host: host.into(),
            username: username.into(),
            password: password.into()
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

    fn request(&self, method: Method, endpoint: &str) -> Result<String> {
        let cli = Client::new();
        let url = &format!("{}/rest/api/latest{}", self.host, endpoint);
        let req = cli.request(method, url)
            .header(ContentType::json())
            .header(Authorization(
                Basic {
                    username: self.username.clone(),
                    password: Some(self.password.clone())
                })
                    );
        let mut res = try!(req.send());
        let mut buf = String::new();
        try!(res.read_to_string(&mut buf));
        debug!("{:?}", buf);
        Ok(buf)
    }
}


#[test]
fn it_works() {
}
