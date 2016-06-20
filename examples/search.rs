extern crate env_logger;
extern crate goji;
extern crate hyper;

use hyper::Client;
use goji::{Credentials, Jira, SearchOptions};
use std::env;

fn main() {
    env_logger::init().unwrap();
    if let (Ok(host), Ok(user), Ok(pass)) = (env::var("JIRA_HOST"), env::var("JIRA_USER"), env::var("JIRA_PASS")) {
        let query = env::args().nth(1).unwrap_or("assignee=doug".to_owned());
        let client = Client::new();
        let jira = Jira::new(host, Credentials::Basic(user, pass), &client);
        let search = jira.search(
            &SearchOptions::builder()
                .jql(query)
                .build()
         );
        if let Ok(results) = search {
            for issue in results.issues {
                println!("{} {} ({}): reporter {} assignee {}",
                         issue.key,
                         issue.summary().unwrap_or("???".to_owned()),
                         issue.status().map(|value| value.name).unwrap_or("???".to_owned()),
                         issue.reporter().map(|value| value.display_name).unwrap_or("???".to_owned()),
                         issue.assignee().map(|value| value.display_name).unwrap_or("???".to_owned()));
            }
        }
    }
}
