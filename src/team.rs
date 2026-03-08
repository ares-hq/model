use crate::scoring::{Auto, Endgame, Penalties, Teleop};
use std::time::SystemTime;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Team {
    pub number: u32,
    pub name: String,
    pub sponsors: String,
    pub location: String,

    pub auto: Auto,
    pub teleop: Teleop,
    pub endgame: Endgame,
    pub penalties: Penalties,

    pub last_match: Option<SystemTime>,
    pub last_checked: Option<SystemTime>,
}

impl Team {
    pub fn new(number: u32, name: String) -> Self {
        Self {
            number,
            name,
            ..Default::default()
        }
    }

    pub fn with_scores(number: u32, name: String, auto: u16, teleop: u16, endgame: u16, penalties: u16) -> Self {
        Self {
            number,
            name,
            auto: Auto { score: auto, rank: 0 },
            teleop: Teleop { score: teleop, rank: 0 },
            endgame: Endgame { score: endgame, rank: 0 },
            penalties: Penalties { score: penalties, rank: 0 },
            ..Default::default()
        }
    }

    pub fn total_score(&self) -> u16 {
        self.auto.score + self.teleop.score + self.endgame.score - self.penalties.score
    }

    pub fn update_last_match(&mut self) {
        self.last_match = Some(SystemTime::now());
    }

    pub fn update_last_checked(&mut self) {
        self.last_checked = Some(SystemTime::now());
    }

    pub fn is_valid(&self) -> bool {
        self.number > 0 && !self.name.is_empty()
    }

    pub fn has_played(&self) -> bool {
        self.last_match.is_some()
    }
}