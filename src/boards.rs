// Third party
use url::form_urlencoded;

// Ours
use {Jira, Result, SearchOptions};

#[derive(Debug)]
pub struct Boards {
    jira: Jira,
}

#[derive(Deserialize, Debug)]
pub struct Board {
    #[serde(rename = "self")]
    pub self_link: String,
    pub id: u64,
    pub name: String,
    #[serde(rename = "type")]
    pub type_name: String,
}

#[derive(Deserialize, Debug)]
pub struct BoardResults {
    #[serde(rename = "maxResults")]
    pub max_results: u64,
    #[serde(rename = "startAt")]
    pub start_at: u64,
    #[serde(rename = "isLast")]
    pub is_last: bool,
    pub values: Vec<Board>,
}

impl Boards {
    pub fn new(jira: &Jira) -> Boards {
        Boards { jira: jira.clone() }
    }

    pub fn get<I>(&self, id: I) -> Result<Board>
    where
        I: Into<String>,
    {
        self.jira.get(&format!("/board/{}", id.into()))
    }

    /// returns a single page of board results
    /// https://docs.atlassian.com/jira-software/REST/latest/#agile/1.0/board-getAllBoards
    pub fn list(&self, options: &SearchOptions) -> Result<BoardResults> {
        let mut path = vec!["/board".to_owned()];
        let query_options = options.serialize().unwrap_or_default();
        let query = form_urlencoded::Serializer::new(query_options).finish();

        path.push(query);

        self.jira.get::<BoardResults>(path.join("?").as_ref())
    }

    /// runs a type why may be used to iterate over consecutive pages of results
    /// https://docs.atlassian.com/jira-software/REST/latest/#agile/1.0/board-getAllBoards
    pub fn iter<'a>(&self, options: &'a SearchOptions) -> Result<BoardsIter<'a>> {
        BoardsIter::new(options, &self.jira)
    }
}

/// provides an iterator over multiple pages of search results
#[derive(Debug)]
pub struct BoardsIter<'a> {
    jira: Jira,
    results: BoardResults,
    search_options: &'a SearchOptions,
}

impl<'a> BoardsIter<'a> {
    fn new(options: &'a SearchOptions, jira: &Jira) -> Result<Self> {
        let results = jira.boards().list(options)?;
        Ok(BoardsIter {
            jira: jira.clone(),
            results,
            search_options: options,
        })
    }

    fn more(&self) -> bool {
        self.results.is_last
    }
}

impl<'a> Iterator for BoardsIter<'a> {
    type Item = Board;
    fn next(&mut self) -> Option<Board> {
        self.results.values.pop().or_else(|| {
            if self.more() {
                match self.jira.boards().list(&self.search_options
                    .as_builder()
                    .max_results(self.results.max_results)
                    .start_at(self.results.start_at + self.results.max_results)
                    .build())
                {
                    Ok(new_results) => {
                        self.results = new_results;
                        self.results.values.pop()
                    }
                    _ => None,
                }
            } else {
                None
            }
        })
    }
}
