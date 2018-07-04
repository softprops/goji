// Third party
use std::collections::BTreeMap;

// Ours
use {Jira, Result};

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
    #[serde(rename = "additionalProperties")]
    pub additional_properties: bool,
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
