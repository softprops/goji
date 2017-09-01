use super::{Jira, Result, Issue};

/// issue options
#[derive(Debug)]
pub struct Issues {
    jira: Jira,
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
}
