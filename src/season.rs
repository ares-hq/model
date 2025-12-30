use std::collections::HashMap;
use crate::event::Event;
use crate::team::Team;

#[derive(Debug, Clone, PartialEq, Default)]
struct Season{
    code: String,
    num_teams: u32,
    num_awards: u32,
    events: HashMap<String, Event>,
    teams: HashMap<u32, Team>,
}


#[derive(Debug, Clone, PartialEq, Default)]
struct Database {
    seasons: HashMap<String, Season>,
}