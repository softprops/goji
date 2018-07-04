# goji [![Build Status](https://travis-ci.org/softprops/goji.svg?branch=master)](https://travis-ci.org/softprops/goji) [![Software License](https://img.shields.io/badge/license-MIT-brightgreen.svg)](LICENSE) [![crates.io](http://meritbadge.herokuapp.com/goji)](https://crates.io/crates/goji) [![Released API docs](https://docs.rs/goji/badge.svg)](http://docs.rs/goji) [![Master API docs](https://img.shields.io/badge/docs-master-green.svg)](https://softprops.github.io/goji)

> a rust interface for [jira](https://www.atlassian.com/software/jira)

## install

Add the following to your `Cargo.toml` file

```toml
[dependencies]
goji = "0.2"
```

## usage

Please browse the [examples](examples/) directory in this repo for some example applications.

Basic usage requires a jira host, and a flavor of `jira::Credentials` for authorization. For user authenticated requests you'll typically want to use `jira::Credentials::Basic` with your jira username and password.

Current support api support is limited to search and issue transitioning.

```rust
extern crate env_logger;
extern crate goji;

use goji::{Credentials, Jira};
use std::env;

fn main() {
    env_logger::init()
    if let (Ok(host), Ok(user), Ok(pass)) =
        (
            env::var("JIRA_HOST"),
            env::var("JIRA_USER"),
            env::var("JIRA_PASS"),
        )
    {
        let query = env::args().nth(1).unwrap_or("assignee=doug".to_owned());

        let jira = Jira::new(host, Credentials::Basic(user, pass)).unwrap();

        match jira.search().iter(query, &Default::default()) {
            Ok(results) => {
                for issue in results {
                    println!("{:#?}", issue);
                }
            }
            Err(err) => panic!("{:#?}", err),
        }
    }
}
```

## what's with the name

Jira's name is a [shortened form of gojira](https://en.wikipedia.org/wiki/Jira_(software)),
another name for godzilla. Goji is a play on that.

Doug Tangren (softprops) 2016-2018
