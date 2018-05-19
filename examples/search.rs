extern crate env_logger;
extern crate goji;

use goji::{Credentials, Jira};
use std::env;

fn main() {
    env_logger::init().unwrap();
    if let (Ok(host), Ok(user), Ok(pass)) = (
        env::var("JIRA_HOST"),
        env::var("JIRA_USER"),
        env::var("JIRA_PASS"),
    ) {
        let query = env::args().nth(1).unwrap_or("assignee=doug".to_owned());

        let jira = Jira::new(host, Credentials::Basic(user, pass)).unwrap();

        match jira.search().iter(query, &Default::default()) {
            Ok(results) => for issue in results {
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
            },
            Err(err) => panic!("{:#?}", err),
        }
    }
}
