use crate::alliance::Alliance;
use crate::r#match::Match;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Event {
    pub code: String,
    pub matches: Vec<Match>,
}

impl Event {
    pub fn new(code: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            matches: Vec::new(),
        }
    }

    pub fn add_match(&mut self, m: Match) {
        self.matches.push(m);
    }

    /// First-appearance order, so OPR columns are reproducible.
    pub fn teams(&self) -> Vec<u32> {
        let mut seen = std::collections::HashSet::new();
        self.matches
            .iter()
            .flat_map(Match::alliances)
            .flat_map(Alliance::present)
            .filter(|&n| seen.insert(n))
            .collect()
    }

    pub fn match_count(&self) -> usize {
        self.matches.len()
    }

    pub fn is_empty(&self) -> bool {
        self.matches.is_empty()
    }
}
