# goji [![Build Status](https://travis-ci.org/softprops/goji.svg?branch=master)](https://travis-ci.org/softprops/goji)

> a rust interface for jira

## docs

[rustdoc](https://softprops.github.io/goji)

## usage

Basic usage requires a jira host, a hyper::Client instance and a flavor of jira::Credentials for authorization. For user authenticated requests you'll typically want to use jira::Credentials::Basic with your jira username and password.

Current support api support is limited to search and issue transitioning.

```rust
extern crate goji;
extern crate hyper;

use hyper::Client;
use goji::{Credentials, Jira, SearchOptions};

fn main() {
    env_logger::init().unwrap();
    if let (Ok(host), Ok(user), Ok(pass)) = (env::var("JIRA_HOST"), env::var("JIRA_USER"), env::var("JIRA_PASS")) {
        let query = env::args().nth(1).unwrap();
        let client = Client::new();
        let jira = Jira::new(host, Credentials::Basic(user, pass), &client);
        let results = jira.search().list(
            &SearchOptions::builder()
                .jql(query)
                .build()
         ).unwrap()
          for issue in results.unwrap().issues {
            println!("{:#?}", issue)
         }
    }
}
```

Doug Tangren (softprops) 2016
