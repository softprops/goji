use tracing::{error, Level};

extern crate goji;

use goji::{Credentials, Jira};
use std::env;

fn main() {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::TRACE)
        // builds the subscriber.
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting tracing default failed");

    if let (Ok(host), Ok(token)) = (env::var("JIRA_HOST"), env::var("JIRA_TOKEN")) {
        let query = env::args()
            .nth(1)
            .unwrap_or("assignee in (currentUser()) order by created DESC".to_owned());

        let jira = Jira::new(host, Credentials::Bearer(token)).unwrap();

        match jira.search().iter(query, &Default::default()) {
            Ok(results) => {
                for issue in results {
                    println!(
                        "{} {} ({}): reporter {} assignee {}",
                        issue.key,
                        issue.summary().unwrap_or("???".to_owned()),
                        issue
                            .status()
                            .map(|value| value.name,)
                            .unwrap_or("???".to_owned(),),
                        issue
                            .reporter()
                            .map(|value| value.display_name,)
                            .unwrap_or("???".to_owned(),),
                        issue
                            .assignee()
                            .map(|value| value.display_name,)
                            .unwrap_or("???".to_owned(),)
                    );
                }
            }
            Err(err) => error!("{:#?}", err),
        }
    }
}
