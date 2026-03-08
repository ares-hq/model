use std::collections::HashMap;
use crate::event::Event;
use crate::team::Team;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Season{
    pub code: String,
    pub num_teams: u32,
    pub num_awards: u32,
    pub events: HashMap<String, Event>,
    pub teams: HashMap<u32, Team>,
}

impl Season {
    pub fn new(code: String) -> Self {
        Self {
            code,
            num_teams: 0,
            num_awards: 0,
            events: HashMap::new(),
            teams: HashMap::new(),
        }
    }

    pub fn add_event(&mut self, event_code: String, event: Event) {
        self.events.insert(event_code, event);
    }

    pub fn add_team(&mut self, team: Team) {
        let number = team.number;
        self.teams.insert(number, team);
        self.num_teams = self.teams.len() as u32;
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Database {
    seasons: HashMap<String, Season>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            seasons: HashMap::new(),
        }
    }

    pub fn add_season(&mut self, season: Season) {
        let code = season.code.clone();
        self.seasons.insert(code, season);
    }

    pub fn get_season(&self, code: &str) -> Option<&Season> {
        self.seasons.get(code)
    }

    pub fn get_season_mut(&mut self, code: &str) -> Option<&mut Season> {
        self.seasons.get_mut(code)
    }

    pub fn season_count(&self) -> usize {
        self.seasons.len()
    }

    pub fn has_season(&self, code: &str) -> bool {
        self.seasons.contains_key(code)
    }

    pub fn remove_season(&mut self, code: &str) -> Option<Season> {
        self.seasons.remove(code)
    }
}