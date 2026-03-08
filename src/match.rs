use crate::alliance::Alliance;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Match {
    pub red: Alliance,
    pub blue: Alliance,
}

impl Match {
    pub fn winner(&self) -> Option<&str> {
        let red_score: u16 = self.red.overall();
        let blue_score: u16 = self.blue.overall();
        if red_score > blue_score {
            Some("Red")
        } else if blue_score > red_score {
            Some("Blue")
        } else {
            None
        }
    }

    pub fn is_tie(&self) -> bool {
        self.red.overall() == self.blue.overall()
    }

    pub fn score_difference(&self) -> u16 {
        let red_score = self.red.overall();
        let blue_score = self.blue.overall();
        if red_score > blue_score {
            red_score - blue_score
        } else {
            blue_score - red_score
        }
    }

    pub fn total_score(&self) -> u16 {
        self.red.overall() + self.blue.overall()
    }

    pub fn red_score(&self) -> u16 {
        self.red.overall()
    }

    pub fn blue_score(&self) -> u16 {
        self.blue.overall()
    }

    pub fn is_complete(&self) -> bool {
        self.red.overall() > 0 || self.blue.overall() > 0
    }
}