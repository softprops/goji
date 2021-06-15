//! Interfaces for accessing and managing attachments

// Ours
use crate::{Attachment, Jira, Result};

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
    pub fn get<I>(&self, id: I) -> Result<Attachment>
    where
        I: Into<String>,
    {
        self.jira.get("api", &format!("/attachment/{}", id.into()))
    }

    /// Delete a single attachment
    ///
    /// See this [jira docs](https://docs.atlassian.com/software/jira/docs/api/REST/8.13.8/#api/2/attachment-removeAttachment)
    /// for more information
    pub fn delete<I>(&self, id: I) -> Result<Attachment>
    where
        I: Into<String>,
    {
        self.jira
            .delete("api", &format!("/attachment/{}", id.into()))
    }
}
