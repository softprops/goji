use super::{Jira, Result, Issue};

/// issue transition options
pub struct Issues<'a> {
    jira: &'a Jira<'a>,
}

impl<'a> Issues<'a> {
    pub fn new(jira: &'a Jira<'a>) -> Issues {
        Issues { jira: jira }
    }

    pub fn get<I>(&self, id: I) -> Result<Issue>
        where I: Into<String>
    {
        self.jira.get(&format!("/issue/{}", id.into()))
    }
}
