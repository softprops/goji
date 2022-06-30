extern crate serde_json;
extern crate url;

use gouqi::*;
use std::collections::HashMap;
use url::form_urlencoded;

macro_rules! builder_pattern {
    ($($name:ident: ($param:ident, $value:expr, $query_param:expr, $query_value:expr),)*) => {
    $(
        #[test]
        fn $name() {
            let options = SearchOptionsBuilder::new()
                .$param($value)
                .build();

            let options_str = options.serialize().unwrap();

            let mut expected: HashMap<&str, &str> = HashMap::new();
            expected.insert($query_param, $query_value);

            let expected_str = form_urlencoded::Serializer::new(String::new())
                .extend_pairs(&expected)
                .finish();

            assert_eq!(options_str, expected_str);
        }
    )*
    }
}

builder_pattern! {
    build_pattern_validate: (validate, true, "validateQuery", "true"),
    build_pattern_fields: (fields, vec!["field1", "field2"], "fields", "field1,field2"),
    build_pattern_max_results: (max_results, 50, "maxResults", "50"),
    build_pattern_start_at: (start_at, 10, "startAt", "10"),
    build_pattern_type_name: (type_name, "my_type", "type", "my_type"),
    build_pattern_name: (name, "my_name", "name", "my_name"),
    build_pattern_project_key_or_id: (project_key_or_id, "1234", "projectKeyOrId", "1234"),
    build_pattern_expand: (expand, vec!["expand1", "expand2"], "expand", "expand1,expand2"),
    build_pattern_state: (state, "my_state", "state","my_state"),
    build_pattern_jql: (jql, "project = '1234'", "jql", "project = '1234'"),
    build_pattern_jalidate_query: (validate_query, true, "validateQuery", "true"),
}
