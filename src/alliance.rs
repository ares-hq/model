use crate::team::Team;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Alliance {
    team1: Team,
    team2: Team,
    color: String,
}

impl Alliance {
    pub fn auto(&self) -> f64 {
        self.team1.auto.score + self.team2.auto.score
    }
    
    pub fn teleop(&self) -> f64 {
        self.team1.teleop.score + self.team2.teleop.score
    }
    
    pub fn endgame(&self) -> f64 {
        self.team1.endgame.score + self.team2.endgame.score
    }
    
    pub fn penalties(&self) -> f64 {
        self.team1.penalties.score + self.team2.penalties.score
    }
    
    pub fn overall(&self) -> f64 {
        self.auto() + self.teleop() + self.endgame() - self.penalties()
    }
    
    pub fn names(&self) -> [&str; 2] {
        [&self.team1.name, &self.team2.name]
    }
    
    pub fn numbers(&self) -> [u32; 2] {
        [self.team1.number, self.team2.number]
    }
    
    pub fn scoreboard(&self) -> [f64; 7] {
        let nums = self.numbers();
        [
            nums[0] as f64,
            nums[1] as f64,
            self.auto(),
            self.teleop(),
            self.endgame(),
            self.penalties(),
            self.overall(),
        ]
    }

    pub fn is_empty(&self) -> bool {
        self.scoreboard().iter().sum::<f64>() == 0.0
    }
}