use super::{Jira, Result, SearchOptions, SearchResults, Issue};
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

    pub fn iter<J>(&self, jql: J, options: &SearchOptions) -> Result<Iter> where J: Into<String> {
        Iter::new(jql, options, self.jira)
    }
}

pub struct Iter<'a> {
    jira: &'a Jira<'a>,
    jql: String,
    results: SearchResults
}

impl<'a> Iter<'a> {
    fn new<J>(jql: J, options: &SearchOptions, jira: &'a Jira<'a>) -> Result<Iter<'a>> where J: Into<String> {
        let query = jql.into();
        let results = try!(jira.search().list(query.clone(), options));
        Ok(Iter {
            jira: jira,
            jql: query,
            results: results
        })
    }

    fn more(&self) -> bool {
        (self.results.start_at + self.results.issues.len() as u64) < self.results.total
    }
}

impl <'a> Iterator for Iter<'a> {
    type Item = Issue;
    fn next(&mut self) -> Option<Issue> {
        self.results.issues.pop().or_else(||
            if self.more() {
                match self.jira.search().list(
                    self.jql.clone(),
                    &SearchOptions::builder()
                        .max_results(self.results.max_results)
                        .start_at(self.results.start_at + self.results.max_results)
                        .build()) {
                            Ok(new_results) => {
                                self.results = new_results;
                                self.results.issues.pop()
                            },
                            _ => None
                        }
            } else {
                None
            }
        )
    }
}
