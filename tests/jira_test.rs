extern crate goji;
extern crate serde_json;

use goji::*;

#[test]
fn deserialise_empty_response() {
    let empty_response_str = "null";
    let empty_response: EmptyResponse = serde_json::from_str(empty_response_str).unwrap();

    assert_eq!(format!("{:#?}", empty_response), "EmptyResponse");
}
