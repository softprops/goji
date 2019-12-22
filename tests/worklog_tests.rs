extern crate goji;
extern crate serde_json;

use goji::worklogs::*;

#[test]
fn deserialise_worklog_results() {
    let issue_results_str = r#"{
        "startAt": 0,
        "maxResults": 50,
        "total": 0,
        "worklogs": []
    }"#;

    let results: WorklogResults = serde_json::from_str(issue_results_str).unwrap();

    assert_eq!(results.start_at, 0);
    assert_eq!(results.max_results, 50);
    assert_eq!(results.total, 0);
    assert_eq!(results.worklogs.len(), 0);
}
