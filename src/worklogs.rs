use url::form_urlencoded;
use crate::rep::{Comment, User, Visibility};

// Ours
use crate::{Jira, Result, SearchOptions};

/// issue options
#[derive(Debug)]
pub struct Worklogs {
    jira: Jira,
}

impl Worklogs {
    pub fn new(jira: &Jira) -> Self {
        Worklogs { jira: jira.clone() }
    }

    /// Returns a single page of worklogs, by issue id
    pub fn list<I>(&self, id: I, options: &SearchOptions) -> Result<WorklogResults>
    where
        I: Into<String>,
    {
        let mut path = vec![format!("/issue/{}/worklog", id.into())];
        let query_options = options.serialize().unwrap_or_default();
        let query = form_urlencoded::Serializer::new(query_options)
            .finish();
        path.push(query);
        self.jira
            .get::<WorklogResults>("api", path.join("?").as_ref())
    }

    /// runs a type why may be used to iterate over consecutive pages of results
    pub fn iter<'a, I>(&self, id: I, options: &'a SearchOptions) -> Result<Iter<'a>>
    where
        I: Into<String>,
    {
        Iter::new(id, options, &self.jira)
    }
}

#[derive(Deserialize, Debug)]
pub struct Worklog {
    #[serde(rename = "self")]
    self_link: String,
    author: User,
    #[serde(rename = "updateAuthor")]
    update_author: User,
    comment: Option<Comment>,
    updated: String,
    visibility: Option<Visibility>,
    started: String,
    #[serde(rename = "timeSpent")]
    time_spent: String,
    #[serde(rename = "timeSpentSeconds")]
    time_spent_seconds: u64,
    id: String,
    #[serde(rename = "issueId")]
    issue_id: String,
}

#[derive(Deserialize, Debug)]
pub struct WorklogResults {
    #[serde(rename = "maxResults")]
    pub max_results: u64,
    #[serde(rename = "startAt")]
    pub start_at: u64,
    pub total: u64,
    pub worklogs: Vec<Worklog>,
}

/// provides an iterator over multiple pages of search results
#[derive(Debug)]
pub struct Iter<'a> {
    jira: Jira,
    id: String,
    results: WorklogResults,
    search_options: &'a SearchOptions,
}

impl<'a> Iter<'a> {
    fn new<I>(id: I, options: &'a SearchOptions, jira: &Jira) -> Result<Self>
    where
        I: Into<String>,
    {
        let id = id.into();
        let results = jira.worklogs().list(id.clone(), options)?;
        Ok(Iter {
            jira: jira.clone(),
            id: id,
            results,
            search_options: options,
        })
    }

    fn more(&self) -> bool {
        (self.results.start_at + self.results.worklogs.len() as u64) < self.results.total
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = Worklog;
    fn next(&mut self) -> Option<Worklog> {
        self.results.worklogs.pop().or_else(|| {
            if self.more() {
                match self.jira.worklogs().list(
                    self.id.clone(),
                    &self
                        .search_options
                        .as_builder()
                        .max_results(self.results.max_results)
                        .start_at(self.results.start_at + self.results.max_results)
                        .build(),
                ) {
                    Ok(new_results) => {
                        self.results = new_results;
                        self.results.worklogs.pop()
                    }
                    _ => None,
                }
            } else {
                None
            }
        })
    }
}
