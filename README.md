# goji [![Build Status](https://travis-ci.org/softprops/goji.svg?branch=master)](https://travis-ci.org/softprops/goji)

> a rust interface for jira

## docs

[rustdoc](https://softprops.github.io/goji)

## install

Add the following to your `Cargo.toml` file

```toml
[dependencies]
goji = "0.1"
```

## usage

Please browse the examples directory in this repo for some example applications.

Basic usage requires a jira host, a `hyper::Client` instance and a flavor of `jira::Credentials` for authorization. For user authenticated requests you'll typically want to use `jira::Credentials::Basic` with your jira username and password.

Current support api support is limited to search and issue transitioning.

```rust
extern crate goji;
extern crate hyper;
extern crate hyper_openssl;

use hyper::Client;
use goji::{Credentials, Jira};

fn main() {
    env_logger::init().unwrap();
    if let (Ok(host), Ok(user), Ok(pass)) = (env::var("JIRA_HOST"), env::var("JIRA_USER"), env::var("JIRA_PASS")) {
        let query = env::args().nth(1).unwrap();

        let ssl = OpensslClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);
        let client = Client::with_connector(connector);

        let jira = Jira::new(host, Credentials::Basic(user, pass), &client);

        let results = jira.search().list(query, &Default::default());
          for issue in results.unwrap().issues {
            println!("{:#?}", issue)
         }
    }
}
```

Doug Tangren (softprops) 2016
