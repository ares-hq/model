use crate::scoring::{Auto, Endgame, Penalties, Teleop};
use std::time::SystemTime;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Team {
    pub number: u32,
    pub name: String,
    pub sponsors: String,
    pub location: String,

    pub auto: Auto,
    pub teleop: Teleop,
    pub endgame: Endgame,
    pub penalties: Penalties,

    pub last_match: Option<SystemTime>,
    pub last_checked: Option<SystemTime>,
}