//! Interfaces for searching for issues

// Third party
use url::form_urlencoded;

// Ours
use {Issue, Jira, Result, SearchOptions, SearchResults};

/// search interface
#[derive(Debug)]
pub struct Search {
    jira: Jira,
}

impl Search {
    pub fn new(jira: &Jira) -> Search {
        Search { jira: jira.clone() }
    }

    /// returns a single page of search results
    /// https://docs.atlassian.com/jira/REST/latest/#api/2/search
    pub fn list<J>(&self, jql: J, options: &SearchOptions) -> Result<SearchResults>
    where
        J: Into<String>,
    {
        let mut path = vec!["/search".to_owned()];
        let query_options = options.serialize().unwrap_or_default();
        let query = form_urlencoded::Serializer::new(query_options)
            .append_pair("jql", &jql.into())
            .finish();
        path.push(query);
        self.jira
            .get::<SearchResults>("api", path.join("?").as_ref())
    }

    /// runs a type why may be used to iterate over consecutive pages of results
    /// https://docs.atlassian.com/jira/REST/latest/#api/2/search
    pub fn iter<'a, J>(&self, jql: J, options: &'a SearchOptions) -> Result<Iter<'a>>
    where
        J: Into<String>,
    {
        Iter::new(jql, options, &self.jira)
    }
}

/// provides an iterator over multiple pages of search results
#[derive(Debug)]
pub struct Iter<'a> {
    jira: Jira,
    jql: String,
    results: SearchResults,
    search_options: &'a SearchOptions,
}

impl<'a> Iter<'a> {
    fn new<J>(jql: J, options: &'a SearchOptions, jira: &Jira) -> Result<Self>
    where
        J: Into<String>,
    {
        let query = jql.into();
        let results = jira.search().list(query.clone(), options)?;
        Ok(Iter {
            jira: jira.clone(),
            jql: query,
            results,
            search_options: options,
        })
    }

    fn more(&self) -> bool {
        (self.results.start_at + self.results.issues.len() as u64) < self.results.total
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = Issue;
    fn next(&mut self) -> Option<Issue> {
        self.results.issues.pop().or_else(|| {
            if self.more() {
                match self.jira.search().list(
                    self.jql.clone(),
                    &self.search_options
                        .as_builder()
                        .max_results(self.results.max_results)
                        .start_at(self.results.start_at + self.results.max_results)
                        .build(),
                ) {
                    Ok(new_results) => {
                        self.results = new_results;
                        self.results.issues.pop()
                    }
                    _ => None,
                }
            } else {
                None
            }
        })
    }
}
