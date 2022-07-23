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
            "comment": [
                {
                    "self": "http://www.example.com/jira/rest/api/2/issue/10010/comment/10000",
                    "id": "10000",
                    "author": {
                        "self": "http://www.example.com/jira/rest/api/2/user?username=fred",
                        "name": "fred",
                        "displayName": "Fred F. User",
                        "active": false
                    },
                    "body": "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Pellentesque eget venenatis elit. Duis eu justo eget augue iaculis fermentum. Sed semper quam laoreet nisi egestas at posuere augue semper.",
                    "updateAuthor": {
                        "self": "http://www.example.com/jira/rest/api/2/user?username=fred",
                        "name": "fred",
                        "displayName": "Fred F. User",
                        "active": false
                    },
                    "created": "2017-02-08T17:08:41.328+0000",
                    "updated": "2017-02-08T17:08:41.330+0000",
                    "visibility": {
                        "type": "role",
                        "value": "Administrators"
                    }
                }
            ],
            "resolutiondate": "2018-07-11T16:56:12.000+0000",
            "created": "2018-07-11T10:56:12.000Z",
            "updated": "2018-07-11T12:56:12.000+00:00"
        }
    }"#;

    let credentials = Credentials::Basic("user".to_string(), "pwd".to_string());
    let jira = Jira::new(JIRA_HOST, credentials).unwrap();
    let issue: Issue = serde_json::from_str(issue_str).unwrap();

    let expected_permalink = format!("{}/browse/{}", JIRA_HOST, issue.key);
    let expected_resolution_date = Some(datetime!(2018-07-11 16:56:12.000 +00:00));
    let expected_created_date = Some(datetime!(2018-07-11 10:56:12.000 +00:00));
    let expected_updated_date = Some(datetime!(2018-07-11 12:56:12.000 +00:00));
    let empty_comments: Vec<Comment> = Vec::new();
    let expected_comment_updated_date = Some(datetime!(2017-02-08 17:08:41.328 +00:00));

    assert_eq!(issue.permalink(&jira), expected_permalink);
    assert_eq!(issue.resolution_date(), expected_resolution_date);
    assert_eq!(issue.created(), expected_created_date);
    assert_eq!(issue.updated(), expected_updated_date);
    assert_ne!(issue.comment().len(), empty_comments.len());
    assert_eq!(issue.comment()[0].created, expected_comment_updated_date);
}
