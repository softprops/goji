extern crate serde_json;

use gouqi::sprints::*;
use time::macros::datetime;

#[test]
fn deserialise_sprint() {
    let sprint_str = r#"{
        "id": 72,
        "self": "http://www.example.com/jira/rest/agile/1.0/sprint/73",
        "name": "sprint 2"
    }"#;

    let sprint: Sprint = serde_json::from_str(sprint_str).unwrap();

    assert_eq!(sprint.id, 72u64);
    assert_eq!(sprint.name, "sprint 2");
    assert_eq!(
        sprint.self_link,
        "http://www.example.com/jira/rest/agile/1.0/sprint/73"
    );
    assert_eq!(sprint.state, None);
    assert_eq!(sprint.start_date, None);
    assert_eq!(sprint.end_date, None);
    assert_eq!(sprint.complete_date, None);
    assert_eq!(sprint.origin_board_id, None);
}

#[test]
fn deserialise_sprint_with_optional_fields() {
    let sprint_str = r#"{
        "id": 72,
        "self": "http://www.example.com/jira/rest/agile/1.0/sprint/73",
        "state": "future",
        "name": "sprint 2",
        "startDate": "2015-04-11T15:22:00.000+10:00",
        "endDate": "2015-04-20T01:22:00.000+10:00",
        "completeDate": "2015-04-20T11:04:00.000+10:00",
        "originBoardId": 5
    }"#;

    let sprint: Sprint = serde_json::from_str(sprint_str).unwrap();

    assert_eq!(sprint.id, 72u64);
    assert_eq!(sprint.state, Some("future".to_owned()));
    assert_eq!(sprint.name, "sprint 2");
    assert_eq!(
        sprint.self_link,
        "http://www.example.com/jira/rest/agile/1.0/sprint/73"
    );
    assert_eq!(
        sprint.start_date,
        Some(datetime!(2015-04-11 15:22:00.000 +10:00))
    );

    assert_eq!(
        sprint.end_date,
        Some(datetime!(2015-04-20 01:22:00.000 +10:00))
    );
    assert_eq!(
        sprint.complete_date,
        Some(datetime!(2015-04-20 11:04:00.000 +10:00))
    );
    assert_eq!(sprint.origin_board_id, Some(5));
}

#[test]
fn deserialise_sprint_results() {
    let sprint_results_str = r#"{
        "maxResults": 50,
        "startAt": 0,
        "isLast": true,
        "values": [{
            "id": 72,
            "self": "http://www.example.com/jira/rest/agile/1.0/sprint/73",
            "state": "future",
            "name": "sprint 2"
        }]
    }"#;

    let sprint_results: SprintResults = serde_json::from_str(sprint_results_str).unwrap();

    assert_eq!(sprint_results.max_results, 50u64);
    assert_eq!(sprint_results.start_at, 0u64);
    assert!(sprint_results.is_last);
    assert_eq!(sprint_results.values.len(), 1);
}
