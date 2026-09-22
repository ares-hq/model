use crate::alliance::Alliance;

const TIE_EPSILON: f64 = 0.01;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Winner {
    Red,
    Blue,
}

impl Winner {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Red => "Red",
            Self::Blue => "Blue",
        }
    }
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

    /// `(red, blue)`, each carrying the fouls its opponent committed.
    pub fn totals(&self) -> (f64, f64) {
        (
            self.red.points() + self.blue.penalties,
            self.blue.points() + self.red.penalties,
        )
    }

    /// `None` on a tie.
    pub fn winner(&self) -> Option<Winner> {
        let (red, blue) = self.totals();
        if (red - blue).abs() < TIE_EPSILON {
            None
        } else if red > blue {
            Some(Winner::Red)
        } else {
            Some(Winner::Blue)
        }
    }

    /// Red, then blue.
    pub fn alliances(&self) -> [&Alliance; 2] {
        [&self.red, &self.blue]
    }
}
