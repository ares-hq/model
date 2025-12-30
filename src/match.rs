use crate::alliance::Alliance;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Match {
    pub red: Alliance,
    pub blue: Alliance,
}

const CATEGORIES: [&str; 7] = [
    "Team 1",
    "Team 2",
    "Auto",
    "Teleop",
    "Endgame",
    "Penalties",
    "Overall",
];

impl Match {
    pub fn winner(&self) -> Option<&str> {
        let red_score: f64 = self.red.overall();
        let blue_score: f64 = self.blue.overall();
        if red_score > blue_score {
            Some("Red")
        } else if blue_score > red_score {
            Some("Blue")
        } else {
            None
        }
    }
}