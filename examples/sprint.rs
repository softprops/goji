use tracing::{error, Level};

extern crate gouqi;

use gouqi::{Credentials, Jira, Sprints};
use std::env;

fn main() {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting tracing default failed");

    if let (Ok(host), Ok(user), Ok(password)) = (
        env::var("JIRA_HOST"),
        env::var("JIRA_USER"),
        env::var("JIRA_PASS"),
    ) {
        let sprint_id = env::args().nth(1).unwrap_or_else(|| "1".to_owned());

        let jira =
            Jira::new(host, Credentials::Basic(user, password)).expect("Error initializing Jira");

        let sprints = Sprints::new(&jira);

        match sprints.get(sprint_id) {
            Ok(sprint) => println!("{:?}", sprint),
            e => error!("{:?}", e),
        }
    } else {
        error!("Missing one or more environment variables JIRA_HOST, JIRA_USER, JIRA_PASS!");
    }
}
