//! Interfaces for accessing and managing issues
//!
// Ours
use {Issue, Jira, Result};

/// issue options
#[derive(Debug)]
pub struct Issues {
    jira: Jira,
}

#[derive(Serialize, Debug)]
pub struct Assignee {
    pub name: String,
}

#[derive(Serialize, Debug)]
pub struct IssueType {
    pub id: String,
}

#[derive(Serialize, Debug)]
pub struct Priority {
    pub id: String,
}

#[derive(Serialize, Debug)]
pub struct Project {
    pub key: String,
}

#[derive(Serialize, Debug)]
pub struct Component {
    pub name: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Fields {
    pub assignee: Assignee,
    pub components: Vec<Component>,
    pub description: String,
    pub environment: String,
    pub issuetype: IssueType,
    pub priority: Priority,
    pub project: Project,
    pub reporter: Assignee,
    pub summary: String,
}

#[derive(Serialize, Debug)]
pub struct CreateIssue {
    pub fields: Fields,
}

#[derive(Debug, Deserialize)]
pub struct CreateResponse {
    pub id: String,
    pub key: String,
    #[serde(rename = "self")]
    pub url: String,
}

impl Issues {
    pub fn new(jira: &Jira) -> Issues {
        Issues { jira: jira.clone() }
    }

    pub fn get<I>(&self, id: I) -> Result<Issue>
    where
        I: Into<String>,
    {
        self.jira.get(&format!("/issue/{}", id.into()))
    }
    pub fn create(&self, data: CreateIssue) -> Result<CreateResponse> {
        self.jira.post("/issue", data)
    }
}
