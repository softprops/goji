extern crate env_logger;
extern crate goji;

use goji::{Credentials, Jira, TransitionTriggerOptions};
use std::env;

fn main() {
    env_logger::init().unwrap();
    if let (Ok(host), Ok(user), Ok(pass), Ok(key)) =
        (
            env::var("JIRA_HOST"),
            env::var("JIRA_USER"),
            env::var("JIRA_PASS"),
            env::var("JIRA_KEY"),
        )
    {

        let jira = Jira::new(host, Credentials::Basic(user, pass)).unwrap();

        println!("{:#?}", jira.issues().get(key.clone()));
        let transitions = jira.transitions(key);
        for option in transitions.list() {
            println!("{:#?}", option);
        }
        if let Ok(transition_id) = env::var("JIRA_TRANSITION_ID") {
            transitions
                .trigger(TransitionTriggerOptions::new(transition_id))
                .unwrap()
        }
    }
}
