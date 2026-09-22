#[derive(Debug, Clone, PartialEq, Default)]
pub struct Alliance {
    /// `0` marks an empty slot.
    pub teams: [u32; 2],
    pub auto: f64,
    pub teleop: f64,
    pub endgame: f64,
    pub penalties: f64,
}

impl Alliance {
    pub fn new(teams: [u32; 2]) -> Self {
        Self {
            teams,
            ..Default::default()
        }
    }

    pub fn points(&self) -> f64 {
        self.auto + self.teleop + self.endgame
    }

    pub fn net_points(&self) -> f64 {
        self.points() - self.penalties
    }

    pub fn present_teams(&self) -> impl Iterator<Item = u32> + '_ {
        self.teams.iter().copied().filter(|&n| n != 0)
    }

    pub fn is_empty(&self) -> bool {
        self.auto == 0.0 && self.teleop == 0.0 && self.endgame == 0.0 && self.penalties == 0.0
    }
}
