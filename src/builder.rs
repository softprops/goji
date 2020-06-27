// Third party
use std::collections::HashMap;
use url::form_urlencoded;

/// Options availble for search
#[derive(Default, Clone, Debug)]
pub struct SearchOptions {
    params: HashMap<&'static str, String>,
}

impl SearchOptions {
    /// Return a new instance of a builder for options
    pub fn builder() -> SearchOptionsBuilder {
        SearchOptionsBuilder::new()
    }

    /// Serialize options as a string. returns None if no options are defined
    pub fn serialize(&self) -> Option<String> {
        if self.params.is_empty() {
            None
        } else {
            Some(
                form_urlencoded::Serializer::new(String::new())
                    .extend_pairs(&self.params)
                    .finish(),
            )
        }
    }

    pub fn as_builder(&self) -> SearchOptionsBuilder {
        SearchOptionsBuilder::copy_from(self)
    }
}

/// A builder interface for search option. Typically this
/// is initialized with SearchOptions::builder()
#[derive(Default, Debug)]
pub struct SearchOptionsBuilder {
    params: HashMap<&'static str, String>,
}

impl SearchOptionsBuilder {
    pub fn new() -> SearchOptionsBuilder {
        SearchOptionsBuilder {
            ..Default::default()
        }
    }

    fn copy_from(search_options: &SearchOptions) -> SearchOptionsBuilder {
        SearchOptionsBuilder {
            params: search_options.params.clone(),
        }
    }

    pub fn fields<F>(&mut self, fs: Vec<F>) -> &mut SearchOptionsBuilder
    where
        F: Into<String>,
    {
        self.params.insert(
            "fields",
            fs.into_iter()
                .map(|f| f.into())
                .collect::<Vec<String>>()
                .join(","),
        );
        self
    }

    pub fn validate(&mut self, v: bool) -> &mut SearchOptionsBuilder {
        self.params.insert("validateQuery", v.to_string());
        self
    }

    pub fn max_results(&mut self, m: u64) -> &mut SearchOptionsBuilder {
        self.params.insert("maxResults", m.to_string());
        self
    }

    pub fn start_at(&mut self, s: u64) -> &mut SearchOptionsBuilder {
        self.params.insert("startAt", s.to_string());
        self
    }

    pub fn type_name(&mut self, t: &str) -> &mut SearchOptionsBuilder {
        self.params.insert("type", t.to_string());
        self
    }

    pub fn name(&mut self, n: &str) -> &mut SearchOptionsBuilder {
        self.params.insert("name", n.to_string());
        self
    }

    pub fn project_key_or_id(&mut self, id: &str) -> &mut SearchOptionsBuilder {
        self.params.insert("projectKeyOrId", id.to_string());
        self
    }

    pub fn expand<E>(&mut self, ex: Vec<E>) -> &mut SearchOptionsBuilder
    where
        E: Into<String>,
    {
        self.params.insert(
            "expand",
            ex.into_iter()
                .map(|e| e.into())
                .collect::<Vec<String>>()
                .join(","),
        );
        self
    }

    pub fn state(&mut self, s: &str) -> &mut SearchOptionsBuilder {
        self.params.insert("state", s.to_string());
        self
    }

    pub fn jql(&mut self, s: &str) -> &mut SearchOptionsBuilder {
        self.params.insert("jql", s.to_string());
        self
    }

    pub fn validate_query(&mut self, v: bool) -> &mut SearchOptionsBuilder {
        self.params.insert("validateQuery", v.to_string());
        self
    }

    pub fn build(&self) -> SearchOptions {
        SearchOptions {
            params: self.params.clone(),
        }
    }
}
