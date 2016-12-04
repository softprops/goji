
use super::{Error, Jira, Result, TransitionOptions, TransitionOption, TransitionTriggerOptions};

/// issue transition interface
pub struct Transitions<'a> {
    jira: &'a Jira<'a>,
    key: String,
}

impl<'a> Transitions<'a> {
    pub fn new<K>(jira: &'a Jira<'a>, key: K) -> Transitions
        where K: Into<String>
    {
        Transitions {
            jira: jira,
            key: key.into(),
        }
    }

    /// return list of transitions options for this issue
    pub fn list(&self) -> Result<Vec<TransitionOption>> {
        self.jira
            .get::<TransitionOptions>(&format!("/issue/{}/transitions?expand=transitions.fields",
                                               self.key))
            .map(|wrapper| wrapper.transitions)
    }

    /// trigger a issue transition
    /// to transition with a resolution use TransitionTrigger::builder(id).resolution(name)
    pub fn trigger(&self, trans: TransitionTriggerOptions) -> Result<()> {
        self.jira
            .post::<(), TransitionTriggerOptions>(&format!("/issue/{}/transitions", self.key), trans)
            .or_else(|e| match e {
                Error::Serde(_) => Ok(()),
                e => Err(e),
            })
    }
}
