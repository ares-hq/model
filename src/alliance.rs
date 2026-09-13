/// One alliance's scores in a match — a row of the OPR system; teams by number.
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

    pub fn overall(&self) -> f64 {
        self.auto + self.teleop + self.endgame - self.penalties
    }

    pub fn has_team(&self, number: u32) -> bool {
        number != 0 && self.teams.contains(&number)
    }

    pub fn present(&self) -> impl Iterator<Item = u32> + '_ {
        self.teams.iter().copied().filter(|&n| n != 0)
    }

    /// Nothing scored, e.g. unplayed.
    pub fn is_empty(&self) -> bool {
        self.overall() == 0.0
    }
}
