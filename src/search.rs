use super::{Jira, Result, SearchOptions, SearchResults, SearchIter};
use url::form_urlencoded;

// search interface
pub struct Search<'a> {
    jira: &'a Jira<'a>,
}

impl<'a> Search<'a> {
    pub fn new(jira: &'a Jira<'a>) -> Search<'a> {
        Search { jira: jira }
    }

    /// https://docs.atlassian.com/jira/REST/latest/#api/2/search
    pub fn list<J>(&self, jql: J, options: &SearchOptions) -> Result<SearchResults>
        where J: Into<String>
    {
        let mut path = vec!["/search".to_owned()];
        let query_options = options.serialize().unwrap_or(String::new());
        let query = form_urlencoded::Serializer::new(query_options)
            .append_pair("jql", &jql.into())
            .finish();
        path.push(query);
        self.jira.get::<SearchResults>(path.join("?").as_ref())
    }

    pub fn iter<J>(&self, jql: J, options: &SearchOptions) -> Result<SearchIter> where J: Into<String> {
        SearchIter::new(jql, options, self.jira)
    }
}
