use tracing::{error, Level};

extern crate gouqi;

use gouqi::{Credentials, Jira};
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
        let query = env::args()
            .nth(1)
            .unwrap_or_else(|| "order by created DESC".to_owned());

        let jira =
            Jira::new(host, Credentials::Basic(user, password)).expect("Error initializing Jira");

        match jira.search().iter(query, &Default::default()) {
            Ok(results) => {
                for issue in results {
                    println!(
                        "{} {} ({}): reporter {} assignee {}",
                        issue.key,
                        issue.summary().unwrap_or_else(|| "???".to_owned()),
                        issue
                            .status()
                            .map(|value| value.name,)
                            .unwrap_or_else(|| "???".to_owned()),
                        issue
                            .reporter()
                            .map(|value| value.display_name,)
                            .unwrap_or_else(|| "???".to_owned()),
                        issue
                            .assignee()
                            .map(|value| value.display_name,)
                            .unwrap_or_else(|| "???".to_owned())
                    );
                }
            }
            Err(err) => error!("{:#?}", err),
        }
    } else {
        error!("Missing one or more environment variables JIRA_HOST, JIRA_USER, JIRA_PASS!");
    }
}
