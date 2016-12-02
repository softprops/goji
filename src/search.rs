use super::{Jira, Result, SearchOptions, SearchResults};

// search interface
pub struct Search<'a> {
    jira: &'a Jira<'a>,
}

impl<'a> Search<'a> {
    pub fn new(jira: &'a Jira<'a>) -> Search<'a> {
        Search { jira: jira }
    }

    /// https://docs.atlassian.com/jira/REST/latest/#api/2/search
    pub fn list(&self, opts: &SearchOptions) -> Result<SearchResults> {
        let mut path = vec!["/search".to_owned()];
        if let Some(q) = opts.serialize() {
            path.push(q);
        }
        self.jira.get::<SearchResults>(path.join("?").as_ref())
    }
}
