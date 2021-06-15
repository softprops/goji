//! Interfaces for accessing and managing attachments

use std::collections::BTreeMap;

// Ours
use crate::{Jira, Result};

/// Same as `User`, but without `email_address`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserResponse {
    pub active: bool,
    #[serde(rename = "avatarUrls")]
    pub avatar_urls: BTreeMap<String, String>,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub key: Option<String>,
    pub name: String,
    #[serde(rename = "self")]
    pub self_link: String,
    #[serde(rename = "timeZone")]
    pub timezone: Option<String>,
}

/// Same as `Attachement`, but without `id` and with `UserResponse`
#[derive(Serialize, Deserialize, Debug)]
pub struct AttachmentResponse {
    #[serde(rename = "self")]
    pub self_link: String,
    pub filename: String,
    pub author: UserResponse,
    pub created: String,
    pub size: u64,
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    pub content: String,
    pub thumbnail: Option<String>,
}

#[derive(Debug)]
pub struct Attachments {
    jira: Jira,
}

impl Attachments {
    pub fn new(jira: &Jira) -> Attachments {
        Attachments { jira: jira.clone() }
    }

    /// Get the meta-data of a single attachment
    ///
    /// See this [jira docs](https://docs.atlassian.com/software/jira/docs/api/REST/8.13.8/#api/2/attachment-getAttachment)
    /// for more information
    pub fn get<I>(&self, id: I) -> Result<AttachmentResponse>
    where
        I: Into<String>,
    {
        self.jira.get("api", &format!("/attachment/{}", id.into()))
    }

    /// Delete a single attachment
    ///
    /// See this [jira docs](https://docs.atlassian.com/software/jira/docs/api/REST/8.13.8/#api/2/attachment-removeAttachment)
    /// for more information
    pub fn delete<I>(&self, id: I) -> Result<AttachmentResponse>
    where
        I: Into<String>,
    {
        self.jira
            .delete("api", &format!("/attachment/{}", id.into()))
    }
}
