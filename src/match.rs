use crate::alliance::Alliance;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Winner {
    Red,
    Blue,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Match {
    pub red: Alliance,
    pub blue: Alliance,
}

impl Match {
    pub fn new(red: Alliance, blue: Alliance) -> Self {
        Self { red, blue }
    }

    /// `None` on a tie.
    pub fn winner(&self) -> Option<Winner> {
        let (red, blue) = (self.red.overall(), self.blue.overall());
        if red > blue {
            Some(Winner::Red)
        } else if blue > red {
            Some(Winner::Blue)
        } else {
            None
        }
    }

    pub fn is_tie(&self) -> bool {
        self.winner().is_none()
    }

    pub fn score_difference(&self) -> f64 {
        (self.red.overall() - self.blue.overall()).abs()
    }

    pub fn total_score(&self) -> f64 {
        self.red.overall() + self.blue.overall()
    }

    pub fn is_complete(&self) -> bool {
        !self.red.is_empty() || !self.blue.is_empty()
    }

    /// Red, then blue.
    pub fn alliances(&self) -> [&Alliance; 2] {
        [&self.red, &self.blue]
    }
}
