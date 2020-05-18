//! Interfaces for accessing and managing the backlog

use crate::{EmptyResponse, Jira, Result};

#[derive(Debug, Serialize)]
struct BacklogIssues {
    issues: Vec<String>,
}

#[derive(Debug)]
pub struct Backlog {
    jira: Jira,
}

impl Backlog {
    pub fn new(jira: &Jira) -> Self {
        Backlog { jira: jira.clone() }
    }

    // See https://docs.atlassian.com/jira-software/REST/7.0.4/#agile/1.0/backlog
    pub fn put(&self, issues: Vec<String>) -> Result<EmptyResponse> {
        let data = BacklogIssues { issues };

        self.jira.post("agile", "/backlog/issue", data)
    }
}

#[cfg(test)]
mod test {
    extern crate serde_json;

    use super::*;

    #[test]
    fn serialise_backlog_issue() {
        let backlog = BacklogIssues {
            issues: vec!["PR-1".to_owned(), "10001".to_owned(), "PR-3".to_owned()],
        };

        let result: String = serde_json::to_string(&backlog).unwrap();
        let expected = r#"{"issues":["PR-1","10001","PR-3"]}"#;

        assert_eq!(result, expected);
    }

}
