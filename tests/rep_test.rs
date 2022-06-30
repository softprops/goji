extern crate gouqi;
extern crate serde_json;

use gouqi::*;
use time::macros::datetime;

const JIRA_HOST: &str = "http://jira.com";

#[test]
fn issue_getters() {
    let issue_str = r#"{
        "self": "https://jira.com/rest/agile/1.0/issue/1234",
        "id": "1234",
        "key": "MYPROJ-1234",
        "fields": {
            "resolutiondate": "2018-07-11T16:56:12.000+0000"
        }
    }"#;

    let credentials = Credentials::Basic("user".to_string(), "pwd".to_string());
    let jira = Jira::new(JIRA_HOST, credentials).unwrap();
    let issue: Issue = serde_json::from_str(issue_str).unwrap();

    let expected_permalink = format!("{}/browse/{}", JIRA_HOST, issue.key);
    let expected_resolution_date = Some(datetime!(2018-07-11 16:56:12.000 +00:00));

    assert_eq!(issue.permalink(&jira), expected_permalink);
    assert_eq!(issue.resolution_date(), expected_resolution_date);
}
