extern crate env_logger;
extern crate goji;
extern crate hyper;

use hyper::Client;
use goji::{Credentials, Jira, SearchOptions};
use std::env;

fn main() {
    env_logger::init().unwrap();
    if let (Some(host), Some(user), Some(pass)) = (env::args().nth(1), env::args().nth(2), env::args().nth(3)) {
        let client = Client::new();
        let jira = Jira::new(host, Credentials::Basic(user, pass), &client);
        println!("{:#?}", jira.search(
            &SearchOptions::builder()
                .jql("assignee=doug")
                .build()
            )
        );
    }
}
