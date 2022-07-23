// Third party

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::collections::BTreeMap;
use time::{format_description::well_known::Iso8601, OffsetDateTime};
use tracing::error;

// Ours
use crate::{Jira, Result};

/// Represents an general jira error response
#[derive(Serialize, Deserialize, Debug)]
pub struct Errors {
    #[serde(rename = "errorMessages")]
    pub error_messages: Vec<String>,
    pub errors: BTreeMap<String, String>,
}

/// Represents a single jira issue
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Issue {
    #[serde(rename = "self")]
    pub self_link: String,
    pub key: String,
    pub id: String,
    pub fields: BTreeMap<String, ::serde_json::Value>,
    pub changelog: Option<Changelog>,
}

impl Issue {
    /// Resolves a typed field from an issues lists of arbitrary fields
    pub fn field<F>(&self, name: &str) -> Option<Result<F>>
    where
        for<'de> F: Deserialize<'de>,
    {
        self.fields
            .get(name)
            .map(|value| Ok(serde_json::value::from_value::<F>(value.clone())?))
    }

    fn user_field(&self, name: &str) -> Option<Result<User>> {
        self.field::<User>(name)
    }

    fn string_field(&self, name: &str) -> Option<Result<String>> {
        self.field::<String>(name)
    }

    /// User assigned to issue
    pub fn assignee(&self) -> Option<User> {
        self.user_field("assignee").and_then(|value| value.ok())
    }

    /// User that created the issue
    pub fn creator(&self) -> Option<User> {
        self.user_field("creator").and_then(|value| value.ok())
    }

    /// User that reported the issue
    pub fn reporter(&self) -> Option<User> {
        self.user_field("reporter").and_then(|value| value.ok())
    }

    /// The current status of the issue
    pub fn status(&self) -> Option<Status> {
        self.field::<Status>("status").and_then(|value| value.ok())
    }

    /// Brief summary of the issue
    pub fn summary(&self) -> Option<String> {
        self.string_field("summary").and_then(|value| value.ok())
    }

    /// Description of the issue
    pub fn description(&self) -> Option<String> {
        self.string_field("description")
            .and_then(|value| value.ok())
    }

    fn extract_offset_date_time(&self, field: &str) -> Option<OffsetDateTime> {
        match self.string_field(field) {
            Some(Ok(created)) => match OffsetDateTime::parse(created.as_ref(), &Iso8601::DEFAULT) {
                Ok(offset_date_time) => Some(offset_date_time),
                Err(error) => {
                    error!(
                        "Can't convert '{} = {:?}' into a OffsetDateTime. {:?}",
                        field, created, error
                    );
                    None
                }
            },
            _ => None,
        }
    }

    /// Updated timestamp
    pub fn updated(&self) -> Option<OffsetDateTime> {
        self.extract_offset_date_time("updated")
    }

    /// Created timestamp
    pub fn created(&self) -> Option<OffsetDateTime> {
        self.extract_offset_date_time("created")
    }

    pub fn resolution_date(&self) -> Option<OffsetDateTime> {
        self.extract_offset_date_time("resolutiondate")
    }

    /// An issue type
    pub fn issue_type(&self) -> Option<IssueType> {
        self.field::<IssueType>("issuetype")
            .and_then(|value| value.ok())
    }

    /// Labels associated with the issue
    pub fn labels(&self) -> Vec<String> {
        self.field::<Vec<String>>("labels")
            .and_then(|value| value.ok())
            .unwrap_or_default()
    }

    /// List of versions associated with the issue
    pub fn fix_versions(&self) -> Vec<Version> {
        self.field::<Vec<Version>>("fixVersions")
            .and_then(|value| value.ok())
            .unwrap_or_default()
    }

    /// Priority of the issue
    pub fn priority(&self) -> Option<Priority> {
        self.field::<Priority>("priority")
            .and_then(|value| value.ok())
    }

    /// Links to other issues
    pub fn links(&self) -> Option<Result<Vec<IssueLink>>> {
        self.field::<Vec<IssueLink>>("issuelinks") //.and_then(|value| value.ok()).unwrap_or(vec![])
    }

    pub fn project(&self) -> Option<Project> {
        self.field::<Project>("project")
            .and_then(|value| value.ok())
    }

    pub fn resolution(&self) -> Option<Resolution> {
        self.field::<Resolution>("resolution")
            .and_then(|value| value.ok())
    }

    pub fn attachment(&self) -> Vec<Attachment> {
        self.field::<Vec<Attachment>>("attachment")
            .and_then(|value| value.ok())
            .unwrap_or_default()
    }

    pub fn comment(&self) -> Vec<Comment> {
        self.field::<Vec<Comment>>("comment")
            .and_then(|value| value.ok())
            .map(|value| value)
            .unwrap_or_default()
    }

    pub fn parent(&self) -> Option<Issue> {
        self.field::<Issue>("parent").and_then(|value| value.ok())
    }

    pub fn timetracking(&self) -> Option<TimeTracking> {
        self.field::<TimeTracking>("timetracking")
            .and_then(|value| value.ok())
    }

    pub fn permalink(&self, jira: &Jira) -> String {
        //format!("{}/browse/{}", jira.host, self.key)
        jira.host
            .join("/browse/")
            .unwrap()
            .join(&self.key)
            .unwrap()
            .to_string()
    }

    pub fn try_from_custom_issue<S: Serialize>(custom_issue: &S) -> serde_json::Result<Self> {
        let serialized_data = serde_json::to_string(custom_issue)?;
        serde_json::from_str(&serialized_data)
    }

    pub fn try_to_custom_issue<D: DeserializeOwned>(&self) -> serde_json::Result<D> {
        let serialized_data = serde_json::to_string(self)?;
        serde_json::from_str(&serialized_data)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Attachment {
    pub id: String,
    #[serde(rename = "self")]
    pub self_link: String,
    pub filename: String,
    pub author: User,
    pub created: String,
    pub size: u64,
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    pub content: String,
    pub thumbnail: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Comment {
    pub id: Option<String>,
    #[serde(rename = "self")]
    pub self_link: String,
    pub author: Option<User>,
    #[serde(rename = "updateAuthor")]
    pub update_author: Option<User>,
    #[serde(default, with = "time::serde::iso8601::option")]
    pub created: Option<OffsetDateTime>,
    #[serde(default, with = "time::serde::iso8601::option")]
    pub updated: Option<OffsetDateTime>,
    pub body: String,
    pub visibility: Option<Visibility>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Visibility {
    #[serde(rename = "type")]
    pub visibility_type: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Changelog {
    pub histories: Vec<History>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct History {
    pub author: User,
    pub created: String,
    pub items: Vec<HistoryItem>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HistoryItem {
    pub field: String,
    pub from: Option<String>,
    #[serde(rename = "fromString")]
    pub from_string: Option<String>,
    pub to: Option<String>,
    #[serde(rename = "toString")]
    pub to_string: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub id: String,
    pub key: String,
    pub name: String,
}

/// Represents link relationship between issues
#[derive(Serialize, Deserialize, Debug)]
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

/// Represents type of issue relation
#[derive(Serialize, Deserialize, Debug)]
pub struct LinkType {
    pub id: String,
    pub inward: String,
    pub name: String,
    pub outward: String,
    #[serde(rename = "self")]
    pub self_link: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Version {
    pub archived: bool,
    pub id: String,
    pub name: String,
    #[serde(rename = "projectId")]
    pub project_id: u64,
    pub released: bool,
    #[serde(rename = "self")]
    pub self_link: String,
}

#[derive(Serialize, Debug)]
pub struct VersionCreationBody {
    pub name: String,
    #[serde(rename = "projectId")]
    pub project_id: u64,
}

#[derive(Serialize, Debug)]
pub struct VersionMoveAfterBody {
    pub after: String,
}

#[derive(Serialize, Debug)]
pub struct VersionUpdateBody {
    pub released: bool,
    pub archived: bool,
    #[serde(rename = "moveUnfixedIssuesTo")]
    pub move_unfixed_issues_to: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub active: bool,
    #[serde(rename = "avatarUrls")]
    pub avatar_urls: Option<BTreeMap<String, String>>,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "emailAddress")]
    pub email_address: Option<String>,
    pub key: Option<String>,
    pub name: String,
    #[serde(rename = "self")]
    pub self_link: String,
    #[serde(rename = "timeZone")]
    pub timezone: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Status {
    pub description: String,
    #[serde(rename = "iconUrl")]
    pub icon_url: String,
    pub id: String,
    pub name: String,
    #[serde(rename = "self")]
    pub self_link: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Priority {
    pub icon_url: String,
    pub id: String,
    pub name: String,
    #[serde(rename = "self")]
    pub self_link: String,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResults {
    pub total: u64,
    #[serde(rename = "maxResults")]
    pub max_results: u64,
    #[serde(rename = "startAt")]
    pub start_at: u64,
    pub expand: Option<String>,
    pub issues: Vec<Issue>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeTracking {
    pub original_estimate: Option<String>,
    pub original_estimate_seconds: Option<u64>,
    pub remaining_estimate: Option<String>,
    pub remaining_estimate_seconds: Option<u64>,
    pub time_spent: Option<String>,
    pub time_spent_seconds: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransitionOption {
    pub id: String,
    pub name: String,
    pub to: TransitionTo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransitionTo {
    pub name: String,
    pub id: String,
}

/// Contains list of options an issue can transitions through
#[derive(Serialize, Deserialize, Debug)]
pub struct TransitionOptions {
    pub transitions: Vec<TransitionOption>,
}

#[derive(Serialize, Debug)]
pub struct TransitionTriggerOptions {
    pub transition: Transition,
    pub fields: BTreeMap<String, ::serde_json::Value>,
}

impl TransitionTriggerOptions {
    /// Creates a new instance
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
    /// Creates a new instance
    pub fn new<I>(id: I) -> TransitionTriggerOptionsBuilder
    where
        I: Into<String>,
    {
        TransitionTriggerOptionsBuilder {
            transition: Transition { id: id.into() },
            fields: BTreeMap::new(),
        }
    }

    /// Appends a field to update as part of transition
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

    /// Updates resolution in transition
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Session {
    pub name: String,
}
