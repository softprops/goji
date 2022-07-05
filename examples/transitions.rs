extern crate gouqi;

use gouqi::{Credentials, Jira, TransitionTriggerOptions};
use std::env;

fn main() {
    // Initialize tracing global tracing subscriber
    use tracing_subscriber::prelude::*;
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        // Use RUST_LOG environment variable to set the tracing level
        .with(tracing_subscriber::EnvFilter::from_default_env())
        // Sets this to be the default, global collector for this application.
        .init();

    if let (Ok(host), Ok(user), Ok(pass), Ok(key)) = (
        env::var("JIRA_HOST"),
        env::var("JIRA_USER"),
        env::var("JIRA_PASS"),
        env::var("JIRA_KEY"),
    ) {
        let jira = Jira::new(host, Credentials::Basic(user, pass)).unwrap();

        println!("{:#?}", jira.issues().get(key.clone()));
        let transitions = jira.transitions(key);
        if let Ok(option) = transitions.list() {
            println!("{:#?}", option);
        }
        if let Ok(transition_id) = env::var("JIRA_TRANSITION_ID") {
            transitions
                .trigger(TransitionTriggerOptions::new(transition_id))
                .unwrap()
        }
    }
}
