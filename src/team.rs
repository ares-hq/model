use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// Season record; fields are the `season_<year>` columns.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Team {
    #[serde(rename = "teamNumber")]
    pub number: u32,
    #[serde(rename = "teamName")]
    pub name: String,
    pub sponsors: String,
    pub location: String,
    /// Rookie year.
    pub founded: Option<u16>,
    pub website: Option<String>,

    #[serde(rename = "autoOPR")]
    pub auto: f64,
    #[serde(rename = "teleOPR")]
    pub teleop: f64,
    #[serde(rename = "endgameOPR")]
    pub endgame: f64,
    pub penalties: f64,
    /// `auto` + `teleop`.
    #[serde(rename = "overallOPR")]
    pub overall: f64,

    #[serde(rename = "autoRank")]
    pub auto_rank: Option<u32>,
    #[serde(rename = "teleRank")]
    pub tele_rank: Option<u32>,
    #[serde(rename = "endgameRank")]
    pub endgame_rank: Option<u32>,
    #[serde(rename = "penaltyRank")]
    pub penalty_rank: Option<u32>,
    #[serde(rename = "overallRank")]
    pub overall_rank: Option<u32>,

    /// Sorted, deduplicated.
    #[serde(rename = "eventsAttended")]
    pub events_attended: Vec<String>,
    #[serde(rename = "profileUpdate")]
    pub profile_update: Option<String>,

    #[serde(skip)]
    pub last_match: Option<SystemTime>,
    #[serde(skip)]
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

    pub fn with_scores(
        number: u32,
        name: String,
        auto: f64,
        teleop: f64,
        endgame: f64,
        penalties: f64,
    ) -> Self {
        let mut team = Self {
            number,
            name,
            auto,
            teleop,
            endgame,
            penalties,
            ..Default::default()
        };
        team.recompute_overall();
        team
    }

    pub fn recompute_overall(&mut self) {
        self.overall = self.auto + self.teleop;
    }

    /// All phases minus penalties, unlike [`Team::overall`].
    pub fn total_score(&self) -> f64 {
        self.auto + self.teleop + self.endgame - self.penalties
    }

    pub fn add_events(&mut self, events: impl IntoIterator<Item = String>) {
        self.events_attended.extend(events);
        self.events_attended.sort();
        self.events_attended.dedup();
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
