extern crate gouqi;

use gouqi::{Credentials, Jira, TransitionTriggerOptions};
use std::env;
use tracing::Level;

fn main() {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::TRACE)
        // builds the subscriber.
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting tracing default failed");

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
