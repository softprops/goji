//! Interfaces for accessing and managing sprints

// Third party
use url::form_urlencoded;

// Ours
use crate::{Board, EmptyResponse, Jira, Result, SearchOptions};

#[derive(Debug)]
pub struct Sprints {
    jira: Jira,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Sprint {
    pub id: u64,
    #[serde(rename = "self")]
    pub self_link: String,
    pub name: String,
    pub state: Option<String>,
    #[serde(rename = "startDate")]
    pub start_date: Option<String>,
    #[serde(rename = "endDate")]
    pub end_date: Option<String>,
    #[serde(rename = "completeDate")]
    pub complete_date: Option<String>,
    #[serde(rename = "originBoardId")]
    pub origin_board_id: Option<u64>,
}

#[derive(Deserialize, Debug)]
pub struct SprintResults {
    #[serde(rename = "maxResults")]
    pub max_results: u64,
    #[serde(rename = "startAt")]
    pub start_at: u64,
    #[serde(rename = "isLast")]
    pub is_last: bool,
    pub values: Vec<Sprint>,
}

#[derive(Serialize, Debug)]
struct MoveIssues {
    issues: Vec<String>,
}

impl Sprints {
    pub fn new(jira: &Jira) -> Sprints {
        Sprints { jira: jira.clone() }
    }

    /// returns a single page of board results
    /// https://docs.atlassian.com/jira-software/REST/latest/#agile/1.0/board/{boardId}/sprint-getAllSprints
    pub fn list(&self, board: &Board, options: &SearchOptions) -> Result<SprintResults> {
        let mut path = vec![format!("/board/{}/sprint", board.id.to_string())];
        let query_options = options.serialize().unwrap_or_default();
        let query = form_urlencoded::Serializer::new(query_options).finish();

        path.push(query);

        self.jira
            .get::<SprintResults>("agile", path.join("?").as_ref())
    }

    /// move issues into sprint
    /// https://docs.atlassian.com/jira-software/REST/7.3.1/#agile/1.0/sprint-moveIssuesToSprint
    pub fn move_issues(&self, sprint_id: u64, issues: Vec<String>) -> Result<EmptyResponse> {
        let path = format!("/sprint/{}/issue", sprint_id);
        let data = MoveIssues { issues };

        self.jira.post("agile", &path, data)
    }
    /// returns a single sprint data
    /// https://docs.atlassian.com/jira-software/REST/7.3.1/#agile/1.0/sprint-getSprint
    pub fn get(&self, sprint_id: u64) -> Result<Sprint> {
        let path = format!("/sprint/{}", sprint_id);

        self.jira.get::<Sprint>("agile", &path)
    }

    /// runs a type why may be used to iterate over consecutive pages of results
    /// https://docs.atlassian.com/jira-software/REST/latest/#agile/1.0/board-getAllBoards
    pub fn iter<'a>(
        &self,
        board: &'a Board,
        options: &'a SearchOptions,
    ) -> Result<SprintsIter<'a>> {
        SprintsIter::new(board, options, &self.jira)
    }
}

/// provides an iterator over multiple pages of search results
#[derive(Debug)]
pub struct SprintsIter<'a> {
    jira: Jira,
    board: &'a Board,
    results: SprintResults,
    search_options: &'a SearchOptions,
}

impl<'a> SprintsIter<'a> {
    fn new(board: &'a Board, options: &'a SearchOptions, jira: &Jira) -> Result<Self> {
        let results = jira.sprints().list(board, options)?;
        Ok(SprintsIter {
            board,
            jira: jira.clone(),
            results,
            search_options: options,
        })
    }

    fn more(&self) -> bool {
        !self.results.is_last
    }
}

impl<'a> Iterator for SprintsIter<'a> {
    type Item = Sprint;
    fn next(&mut self) -> Option<Sprint> {
        self.results.values.pop().or_else(|| {
            if self.more() {
                match self.jira.sprints().list(
                    self.board,
                    &self
                        .search_options
                        .as_builder()
                        .max_results(self.results.max_results)
                        .start_at(self.results.start_at + self.results.max_results)
                        .build(),
                ) {
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
