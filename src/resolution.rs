use super::{Jira, Result};

use std::collections::BTreeMap;
use serde::Deserialize;

#[derive(Debug)]
pub struct Resolution {
    jira: Jira,
}

#[derive(Deserialize, Debug)]
pub struct Resolved {
    pub id: String,
    pub title: String,
    #[serde(rename = "type")]
    pub resolution_type: String,
    pub properties: BTreeMap<String, ::serde_json::Value>,
    pub additionalProperties: bool,
}

impl Resolution {
    pub fn new(jira: &Jira) -> Resolution {
        Resolution { jira: jira.clone() }
    }

    pub fn get<I>(&self, id: I) -> Result<Resolved>
    where
        I: Into<String>,
    {
        self.jira.get(&format!("/resolution/{}", id.into()))
    }
}
