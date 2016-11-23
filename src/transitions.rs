
use super::{Error, Jira, Result, TransitionsWrapper, TransitionData, TransitionWrapper};

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

    /// return list of transitions available to an issue
    pub fn list(&self) -> Result<Vec<TransitionData>> {
        self.jira
            .get::<TransitionsWrapper>(&format!("/issue/{}/transitions?expand=transitions.fields",
                                                self.key))
            .map(|wrapper| wrapper.transitions)
    }

    /// trigger a transition
    pub fn trigger(&self, trans: TransitionWrapper) -> Result<()> {
        self.jira
            .post::<(), TransitionWrapper>(&format!("/issue/{}/transitions", self.key), trans)
            .or_else(|e| match e {
                Error::Serde(_) => Ok(()),
                e => Err(e),
            })
    }
}
