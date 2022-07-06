extern crate gouqi;
extern crate serde_json;

use gouqi::*;

const JIRA_HOST: &str = "http://jira.com";

#[test]
fn jira_new_should_err_if_no_uri() {
    let credentials = Credentials::Basic("user".to_string(), "pwd".to_string());
    let jira = Jira::new("12345", credentials);
    assert!(jira.is_err());
}

#[test]
fn jira_new_should_ok_with_uri() {
    let credentials = Credentials::Basic("user".to_string(), "pwd".to_string());
    let jira = Jira::new(JIRA_HOST, credentials);
    assert!(jira.is_ok());
}
