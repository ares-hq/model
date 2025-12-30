use std::ops::{Add, Sub};

#[derive(Debug, Clone, PartialEq, Default)]
struct Scoring {
    score: f64,
    rank: u32,
}

impl Add for Scoring {
    type Output = Self;

    fn add(self, other: Scoring) -> Self {
        Self {
            score: self.score + other.score,
            rank: 0
        }
    }
}

impl Sub for Scoring {
    type Output = Self;

    fn sub(self, other: Scoring) -> Self {
        Self {
            score: self.score - other.score,
            rank: 0
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Auto {
    pub score: f64,
    pub rank: u32,
}

impl Add for Auto {
    type Output = Self;

    fn add(self, other: Auto) -> Self {
        Self {
            score: self.score + other.score,
            rank: 0
        }
    }
}

impl Sub for Auto {
    type Output = Self;

    fn sub(self, other: Auto) -> Self {
        Self {
            score: self.score - other.score,
            rank: 0
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Teleop {
    pub score: f64,
    pub rank: u32,
}

impl Add for Teleop {
    type Output = Self;

    fn add(self, other: Teleop) -> Self {
        Self {
            score: self.score + other.score,
            rank: 0
        }
    }
}

impl Sub for Teleop {
    type Output = Self;

    fn sub(self, other: Teleop) -> Self {
        Self {
            score: self.score - other.score,
            rank: 0
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Endgame {
    pub score: f64,
    pub rank: u32,
}

impl Add for Endgame {
    type Output = Self;

    fn add(self, other: Endgame) -> Self {
        Self {
            score: self.score + other.score,
            rank: 0
        }
    }
}

impl Sub for Endgame {
    type Output = Self;

    fn sub(self, other: Endgame) -> Self {
        Self {
            score: self.score - other.score,
            rank: 0
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Penalties {
    pub score: f64,
    pub rank: u32,
}

impl Add for Penalties {
    type Output = Self;

    fn add(self, other: Penalties) -> Self {
        Self {
            score: self.score + other.score,
            rank: 0
        }
    }
}

impl Sub for Penalties {
    type Output = Self;

    fn sub(self, other: Penalties) -> Self {
        Self {
            score: self.score - other.score,
            rank: 0
        }
    }
}