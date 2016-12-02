extern crate env_logger;
extern crate goji;
extern crate hyper;

use hyper::Client;
use goji::{Credentials, Jira, TransitionTrigger};
use std::env;

fn main() {
    env_logger::init().unwrap();
    if let (Ok(host), Ok(user), Ok(pass), Ok(key)) = (
        env::var("JIRA_HOST"), env::var("JIRA_USER"), env::var("JIRA_PASS"), env::var("JIRA_KEY")) {
        let client = Client::new();
        let jira = Jira::new(host, Credentials::Basic(user, pass), &client);
        println!("{:#?}", jira.issues().get(key.clone()));
        let transitions = jira.transitions(key);
        for option in transitions.list() {
            println!("{:#?}", option);
        }
        if let Ok(transition_id) = env::var("JIRA_TRANSITION_ID") {
            transitions.trigger(TransitionTrigger::new(transition_id)).unwrap()
        }
    }
}
