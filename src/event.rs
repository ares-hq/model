use crate::r#match::Match;
use crate::team::Team;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Event {
    pub code: String,
    pub teams: Vec<Team>,
    pub matches: HashMap<String, Match>,
}

impl Event {
    pub fn add_team(&mut self, team: Team) {
        self.teams.push(team);
    }

    pub fn add_match(&mut self, match_id: String, match_data: Match) {
        self.matches.insert(match_id, match_data);
    }

    pub fn get_team(&self, number: u32) -> Option<&Team> {
        self.teams.iter().find(|t| t.number == number)
    }

    pub fn get_team_mut(&mut self, number: u32) -> Option<&mut Team> {
        self.teams.iter_mut().find(|t| t.number == number)
    }

    pub fn team_count(&self) -> usize {
        self.teams.len()
    }

    pub fn match_count(&self) -> usize {
        self.matches.len()
    }

    pub fn get_match(&self, match_id: &str) -> Option<&Match> {
        self.matches.get(match_id)
    }

    pub fn is_empty(&self) -> bool {
        self.teams.is_empty() && self.matches.is_empty()
    }
}
