use serde::{Deserialize, Serialize};

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
    /// `auto` + `teleop` only.
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
}

impl Team {
    pub fn new(number: u32, name: String) -> Self {
        Self {
            number,
            name,
            ..Default::default()
        }
    }

    pub fn recompute_overall(&mut self) {
        self.overall = self.auto + self.teleop;
    }

    pub fn add_events(&mut self, events: impl IntoIterator<Item = String>) {
        self.events_attended.extend(events);
        self.events_attended.sort();
        self.events_attended.dedup();
    }
}
