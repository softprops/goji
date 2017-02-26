extern crate env_logger;
extern crate goji;
extern crate hyper;
extern crate hyper_openssl;

use hyper::Client;
use hyper::net::HttpsConnector;
use hyper_openssl::OpensslClient;
use goji::{Credentials, Jira, TransitionTriggerOptions};
use std::env;

fn main() {
    env_logger::init().unwrap();
    if let (Ok(host), Ok(user), Ok(pass), Ok(key)) =
        (env::var("JIRA_HOST"), env::var("JIRA_USER"), env::var("JIRA_PASS"), env::var("JIRA_KEY")) {

        let ssl = OpensslClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);
        let client = Client::with_connector(connector);

        let jira = Jira::new(host, Credentials::Basic(user, pass), &client);

        println!("{:#?}", jira.issues().get(key.clone()));
        let transitions = jira.transitions(key);
        for option in transitions.list() {
            println!("{:#?}", option);
        }
        if let Ok(transition_id) = env::var("JIRA_TRANSITION_ID") {
            transitions.trigger(TransitionTriggerOptions::new(transition_id)).unwrap()
        }
    }
}
