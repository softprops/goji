use url::form_urlencoded;
use std::collections::HashMap;

/// options availble for search
#[derive(Default)]
pub struct SearchOptions {
    params: HashMap<&'static str, String>,
}

impl SearchOptions {
    /// return a new instance of a builder for options
    pub fn builder() -> SearchOptionsBuilder {
        SearchOptionsBuilder::new()
    }

    /// serialize options as a string. returns None if no options are defined
    pub fn serialize(&self) -> Option<String> {
        if self.params.is_empty() {
            None
        } else {
            Some(form_urlencoded::Serializer::new(String::new())
                .extend_pairs(&self.params)
                .finish())
        }
    }
}

/// a builder interface for search option
/// Typically this is initialized with SearchOptions::builder()
#[derive(Default)]
pub struct SearchOptionsBuilder {
    params: HashMap<&'static str, String>,
}

impl SearchOptionsBuilder {
    pub fn new() -> SearchOptionsBuilder {
        SearchOptionsBuilder { ..Default::default() }
    }

    pub fn fields<F>(&mut self, fs: Vec<F>) -> &mut SearchOptionsBuilder
        where F: Into<String>
    {
        self.params.insert("fields",
                           fs.into_iter().map(|f| f.into()).collect::<Vec<String>>().join(","));
        self
    }

    pub fn validate(&mut self, v: bool) -> &mut SearchOptionsBuilder {
        self.params.insert("validateQuery", v.to_string());
        self
    }

    pub fn max(&mut self, m: u64) -> &mut SearchOptionsBuilder {
        self.params.insert("maxResults", m.to_string());
        self
    }

    pub fn start_at(&mut self, s: u64) -> &mut SearchOptionsBuilder {
        self.params.insert("startAt", s.to_string());
        self
    }

    pub fn expand<E>(&mut self, ex: Vec<E>) -> &mut SearchOptionsBuilder
        where E: Into<String>
    {
        self.params.insert("expand",
                           ex.into_iter().map(|e| e.into()).collect::<Vec<String>>().join(","));
        self
    }

    pub fn build(&self) -> SearchOptions {
        SearchOptions { params: self.params.clone() }
    }
}
