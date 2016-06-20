extern crate serde;
extern crate serde_json;

use super::Result;
use serde::de::Deserialize;

include!(concat!(env!("OUT_DIR"), "/rep.rs"));
