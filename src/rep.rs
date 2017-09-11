use serde::Deserialize;
use serde::Serialize;
use serde_json;
use std::collections::BTreeMap;
use super::Result;

/// represents an general jira error response
#[derive(Deserialize, Debug)]
pub struct Errors {
    #[serde(rename = "errorMessages")]
    pub error_messages: Vec<String>,
    pub errors: BTreeMap<String, String>,
}

/// represents a single jira issue
#[derive(Deserialize, Debug)]
pub struct Issue {
    #[serde(rename = "self")]
    pub self_link: String,
    pub key: String,
    pub id: String,
    //    pub expand: String,
    pub fields: BTreeMap<String, ::serde_json::Value>,
}

impl Issue {
    /// resolves a typed field from an issues lists of arbitrary fields
    pub fn field<'a, F>(&self, name: &str) -> Option<Result<F>>
    where
        for<'de> F: Deserialize<'de>,
    //where
    //    F: Deserialize<'a>,
    {
        self.fields.get(name).map(|value| {
            let decoded = try!(serde_json::value::from_value::<F>(value.clone()));
            Ok(decoded)
        })
    }

    fn user_field(&self, name: &str) -> Option<Result<User>> {
        self.field::<User>(name)
    }

    fn string_field(&self, name: &str) -> Option<Result<String>> {
        self.field::<String>(name)
    }

    /// user assigned to issue
    pub fn assignee(&self) -> Option<User> {
        self.user_field("assignee").and_then(|value| value.ok())
    }

    /// user that created the issue
    pub fn creator(&self) -> Option<User> {
        self.user_field("creator").and_then(|value| value.ok())
    }

    /// user that reported the issue
    pub fn reporter(&self) -> Option<User> {
        self.user_field("reporter").and_then(|value| value.ok())
    }

    /// the current status of the issue
    pub fn status(&self) -> Option<Status> {
        self.field::<Status>("status").and_then(|value| value.ok())
    }

    /// brief summary of the issue
    pub fn summary(&self) -> Option<String> {
        self.string_field("summary").and_then(|value| value.ok())
    }

    /// description of the issue
    pub fn description(&self) -> Option<String> {
        self.string_field("description").and_then(
            |value| value.ok(),
        )
    }

    /// updated timestamp
    pub fn updated(&self) -> Option<String> {
        self.string_field("updated").and_then(|value| value.ok())
    }

    /// created timestamp
    pub fn created(&self) -> Option<String> {
        self.string_field("created").and_then(|value| value.ok())
    }

    pub fn resolution_date(&self) -> Option<String> {
        self.string_field("resolution_date").and_then(
            |value| value.ok(),
        )
    }

    /// an issue type
    pub fn issue_type(&self) -> Option<IssueType> {
        self.field::<IssueType>("issuetype").and_then(
            |value| value.ok(),
        )
    }

    /// labels associated with the issue
    pub fn labels(&self) -> Vec<String> {
        self.field::<Vec<String>>("labels")
            .and_then(|value| value.ok())
            .unwrap_or(vec![])
    }

    /// list of versions associated with the issue
    pub fn fix_versions(&self) -> Vec<Version> {
        self.field::<Vec<Version>>("fixVersions")
            .and_then(|value| value.ok())
            .unwrap_or(vec![])
    }

    /// priority of the issue
    pub fn priority(&self) -> Option<Priority> {
        self.field::<Priority>("priority").and_then(
            |value| value.ok(),
        )
    }

    /// links to other issues
    pub fn links(&self) -> Option<Result<Vec<IssueLink>>> {
        self.field::<Vec<IssueLink>>("issuelinks") //.and_then(|value| value.ok()).unwrap_or(vec![])
    }

    pub fn project(&self) -> Option<Project> {
        self.field::<Project>("project").and_then(
            |value| value.ok(),
        )
    }

    pub fn resolution(&self) -> Option<Resolution> {
        self.field::<Resolution>("resolution").and_then(
            |value| value.ok(),
        )
    }
}


#[derive(Deserialize, Debug)]
pub struct Project {
    pub id: String,
    pub key: String,
    pub name: String,
}


/// represents link relationship between issues
#[derive(Deserialize, Debug)]
pub struct IssueLink {
    pub id: String,
    #[serde(rename = "self")]
    pub self_link: String,
    #[serde(rename = "outwardIssue")]
    pub outward_issue: Option<Issue>,
    #[serde(rename = "inwardIssue")]
    pub inward_issue: Option<Issue>,
    #[serde(rename = "type")]
    pub link_type: LinkType,
}

/// represents type of issue relation
#[derive(Deserialize, Debug)]
pub struct LinkType {
    pub id: String,
    pub inward: String,
    pub name: String,
    pub outward: String,
    #[serde(rename = "self")]
    pub self_link: String,
}

#[derive(Deserialize, Debug)]
pub struct Version {
    pub archived: bool,
    pub id: String,
    pub name: String,
    pub released: bool,
    #[serde(rename = "self")]
    pub self_link: String,
}

#[derive(Deserialize, Debug)]
pub struct User {
    pub active: bool,
    #[serde(rename = "avatarUrls")]
    pub avatar_urls: BTreeMap<String, String>,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "emailAddress")]
    pub email_address: String,
    pub key: String,
    pub name: String,
    #[serde(rename = "self")]
    pub self_link: String,
    #[serde(rename = "timeZone")]
    pub timezone: String,
}

#[derive(Deserialize, Debug)]
pub struct Status {
    pub description: String,
    #[serde(rename = "iconUrl")]
    pub icon_url: String,
    pub id: String,
    pub name: String,
    #[serde(rename = "self")]
    pub self_link: String,
}

#[derive(Deserialize, Debug)]
pub struct Priority {
    pub icon_url: String,
    pub id: String,
    pub name: String,
    #[serde(rename = "self")]
    pub self_link: String,
}

#[derive(Deserialize, Debug)]
pub struct IssueType {
    pub description: String,
    #[serde(rename = "iconUrl")]
    pub icon_url: String,
    pub id: String,
    pub name: String,
    #[serde(rename = "self")]
    pub self_link: String,
    pub subtask: bool,
}

#[derive(Deserialize, Debug)]
pub struct SearchResults {
    pub total: u64,
    #[serde(rename = "maxResults")]
    pub max_results: u64,
    #[serde(rename = "startAt")]
    pub start_at: u64,
    pub expand: String,
    pub issues: Vec<Issue>,
}

#[derive(Deserialize, Debug)]
pub struct TransitionOption {
    pub id: String,
    pub name: String,
    pub to: TransitionTo,
}

#[derive(Deserialize, Debug)]
pub struct TransitionTo {
    pub name: String,
    pub id: String,
}

/// contains list of options an issue can transitions through
#[derive(Deserialize, Debug)]
pub struct TransitionOptions {
    pub transitions: Vec<TransitionOption>,
}

#[derive(Serialize, Debug)]
pub struct TransitionTriggerOptions {
    pub transition: Transition,
    pub fields: BTreeMap<String, ::serde_json::Value>,
}

impl TransitionTriggerOptions {
    /// creates a new instance
    pub fn new<I>(id: I) -> TransitionTriggerOptions
    where
        I: Into<String>,
    {
        TransitionTriggerOptions {
            transition: Transition { id: id.into() },
            fields: BTreeMap::new(),
        }
    }

    pub fn builder<I>(id: I) -> TransitionTriggerOptionsBuilder
    where
        I: Into<String>,
    {
        TransitionTriggerOptionsBuilder::new(id)
    }
}

pub struct TransitionTriggerOptionsBuilder {
    pub transition: Transition,
    pub fields: BTreeMap<String, ::serde_json::Value>,
}

impl TransitionTriggerOptionsBuilder {
    /// creates a new instance
    pub fn new<I>(id: I) -> TransitionTriggerOptionsBuilder
    where
        I: Into<String>,
    {
        TransitionTriggerOptionsBuilder {
            transition: Transition { id: id.into() },
            fields: BTreeMap::new(),
        }
    }

    /// appends a field to update as part of transition
    pub fn field<N, V>(&mut self, name: N, value: V) -> &mut TransitionTriggerOptionsBuilder
    where
        N: Into<String>,
        V: Serialize,
    {
        self.fields.insert(
            name.into(),
            serde_json::to_value(value).expect("Value to serialize"),
        );
        self
    }

    /// updates resolution in transition
    pub fn resolution<R>(&mut self, name: R) -> &mut TransitionTriggerOptionsBuilder
    where
        R: Into<String>,
    {
        self.field("resolution", Resolution { name: name.into() });
        self
    }

    pub fn build(&self) -> TransitionTriggerOptions {
        TransitionTriggerOptions {
            transition: self.transition.clone(),
            fields: self.fields.clone(),
        }
    }
}

#[derive(Serialize, Debug, Deserialize)]
pub struct Resolution {
    name: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct Transition {
    pub id: String,
}
