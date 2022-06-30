extern crate serde_json;

use gouqi::issues::*;

#[test]
fn deserialise_issue_results() {
    let issue_results_str = r#"{
        "expand": "names,schema",
        "startAt": 0,
        "maxResults": 50,
        "total": 0,
        "issues": []
    }"#;

    let results: IssueResults = serde_json::from_str(issue_results_str).unwrap();

    assert_eq!(results.expand, Some(String::from("names,schema")));
    assert_eq!(results.start_at, 0);
    assert_eq!(results.max_results, 50);
    assert_eq!(results.total, 0);
    assert_eq!(results.issues.len(), 0);
}
