use crate::team::Team;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Alliance {
    pub team1: Team,
    pub team2: Team,
    pub color: String,
}

impl Alliance {
    pub fn new(team1: Team, team2: Team, color: String) -> Self {
        Self { team1, team2, color }
    }

    pub fn auto(&self) -> u16 {
        self.team1.auto.score + self.team2.auto.score
    }
    
    pub fn teleop(&self) -> u16 {
        self.team1.teleop.score + self.team2.teleop.score
    }
    
    pub fn endgame(&self) -> u16 {
        self.team1.endgame.score + self.team2.endgame.score
    }
    
    pub fn penalties(&self) -> u16 {
        self.team1.penalties.score + self.team2.penalties.score
    }
    
    pub fn overall(&self) -> u16 {
        self.auto() + self.teleop() + self.endgame() - self.penalties()
    }
    
    pub fn names(&self) -> [&str; 2] {
        [&self.team1.name, &self.team2.name]
    }
    
    pub fn numbers(&self) -> [u32; 2] {
        [self.team1.number, self.team2.number]
    }
    
    pub fn scoreboard(&self) -> HashMap<&str, u32> {
        let nums = self.numbers();
        HashMap::from([
            ("Team 1", nums[0]),
            ("Team 2", nums[1]),
            ("Auto", self.auto() as u32),
            ("Teleop", self.teleop() as u32),
            ("Endgame", self.endgame() as u32),
            ("Penalties", self.penalties() as u32),
            ("Overall", self.overall() as u32),
        ])
    }

    pub fn is_empty(&self) -> bool {
        self.overall() == 0
    }

    pub fn has_team(&self, number: u32) -> bool {
        self.team1.number == number || self.team2.number == number
    }

    pub fn average_score(&self) -> u16 {
        self.overall() / 2
    }
}