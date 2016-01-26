extern crate hyper;
extern crate rustc_serialize;
extern crate url;

pub mod errors;
pub use errors::Error;
use hyper::client::Client;
use hyper::method::Method;
use hyper::header::{ContentType, Authorization, Basic};
use std::io::Read;

pub mod builder;

pub use builder::SearchOptions;

pub type Result<T> = std::result::Result<T, Error>;

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

    pub fn search(&self, opts: &SearchOptions) -> Result<String> {
        let mut path = vec!["/search".to_owned()];
        if let Some(q) = opts.serialize() {
            path.push(q);
        }
        self.get(path.join("?").as_ref())
    }

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
        Ok(buf)
    }
}


#[test]
fn it_works() {
}
