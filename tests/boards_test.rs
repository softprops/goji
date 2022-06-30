extern crate gouqi;
extern crate serde_json;

use gouqi::boards::*;

#[test]
fn deserialise_board() {
    let board_str = r#"{
        "id": 1,
        "self": "https://my.atlassian.net/rest/agile/1.0/board/1",
        "name": "TEST board",
        "type": "kanban"
    }"#;

    let board: Board = serde_json::from_str(board_str).unwrap();

    assert_eq!(board.id, 1u64);
    assert_eq!(
        board.self_link,
        "https://my.atlassian.net/rest/agile/1.0/board/1"
    );
    assert_eq!(board.name, "TEST board");
    assert_eq!(board.type_name, "kanban");
}

#[test]
fn deserialise_board_results() {
    let board_results_str = r#"{
        "maxResults": 50,
        "startAt": 0,
        "total": 2,
        "isLast": true,
        "values": [{
            "id": 1,
            "self": "https://my.atlassian.net/rest/agile/1.0/board/1",
            "name": "TEST board",
            "type": "kanban"
        }]
    }"#;

    let board_results: BoardResults = serde_json::from_str(board_results_str).unwrap();

    assert_eq!(board_results.max_results, 50u64);
    assert_eq!(board_results.start_at, 0u64);
    assert!(board_results.is_last);
    assert_eq!(board_results.values.len(), 1);
}
