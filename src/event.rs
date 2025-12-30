use crate::team::Team;
use crate::r#match::Match;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Event {
    pub code: String,
    pub teams : Vec<Team>,
    pub matches: HashMap<String, Match>,
}
