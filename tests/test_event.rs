use cache::event::Event;
use cache::team::Team;
use cache::r#match::Match;
use std::collections::HashMap;

#[test]
fn test_event_default() {
    let event = Event::default();
    assert_eq!(event.code, "");
    assert_eq!(event.teams.len(), 0);
    assert_eq!(event.matches.len(), 0);
}

#[test]
fn test_event_with_code() {
    let event = Event {
        code: "TXHOU".to_string(),
        teams: Vec::new(),
        matches: HashMap::new(),
    };
    assert_eq!(event.code, "TXHOU");
}

#[test]
fn test_event_with_teams() {
    let team1 = Team::default();
    let team2 = Team::default();
    
    let event = Event {
        code: "TXHOU".to_string(),
        teams: vec![team1, team2],
        matches: HashMap::new(),
    };
    
    assert_eq!(event.teams.len(), 2);
}

#[test]
fn test_event_with_matches() {
    let mut matches = HashMap::new();
    matches.insert("Q1".to_string(), Match::default());
    matches.insert("Q2".to_string(), Match::default());
    
    let event = Event {
        code: "TXHOU".to_string(),
        teams: Vec::new(),
        matches,
    };
    
    assert_eq!(event.matches.len(), 2);
    assert!(event.matches.contains_key("Q1"));
    assert!(event.matches.contains_key("Q2"));
}

#[test]
fn test_event_clone() {
    let event1 = Event {
        code: "TXHOU".to_string(),
        teams: Vec::new(),
        matches: HashMap::new(),
    };
    let event2 = event1.clone();
    
    assert_eq!(event1, event2);
}

#[test]
fn test_event_debug_format() {
    let event = Event::default();
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("Event"));
}

#[test]
fn test_event_partial_eq() {
    let event1 = Event {
        code: "TXHOU".to_string(),
        teams: Vec::new(),
        matches: HashMap::new(),
    };
    let event2 = Event {
        code: "TXHOU".to_string(),
        teams: Vec::new(),
        matches: HashMap::new(),
    };
    
    assert_eq!(event1, event2);
}

#[test]
fn test_event_add_match() {
    let mut event = Event::default();
    event.matches.insert("Q1".to_string(), Match::default());
    
    assert_eq!(event.matches.len(), 1);
    assert!(event.matches.contains_key("Q1"));
}

#[test]
fn test_event_add_team() {
    let mut event = Event::default();
    event.teams.push(Team::default());
    
    assert_eq!(event.teams.len(), 1);
}

#[test]
fn test_event_realistic_scenario() {
    let mut event = Event {
        code: "TXHOU".to_string(),
        teams: Vec::new(),
        matches: HashMap::new(),
    };
    
    // Add some teams
    for i in 1..=10 {
        let team = Team {
            number: 1000 + i,
            name: format!("Team {}", i),
            ..Default::default()
        };
        event.teams.push(team);
    }
    
    // Add some matches
    for i in 1..=6 {
        event.matches.insert(format!("Q{}", i), Match::default());
    }
    
    assert_eq!(event.teams.len(), 10);
    assert_eq!(event.matches.len(), 6);
}
