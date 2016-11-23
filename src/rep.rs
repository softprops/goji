extern crate serde;
extern crate serde_json;

use super::Result;
use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeMap;

include!(concat!(env!("OUT_DIR"), "/rep.rs"));
