use cache::alliance::Alliance;
use cache::scoring::{Auto, Endgame, Penalties, Teleop};
use cache::team::Team;

fn create_test_team(
    number: u32,
    name: &str,
    auto: u16,
    teleop: u16,
    endgame: u16,
    penalties: u16,
) -> Team {
    Team {
        number,
        name: name.to_string(),
        sponsors: String::new(),
        location: String::new(),
        auto: Auto {
            score: auto,
            rank: 0,
        },
        teleop: Teleop {
            score: teleop,
            rank: 0,
        },
        endgame: Endgame {
            score: endgame,
            rank: 0,
        },
        penalties: Penalties {
            score: penalties,
            rank: 0,
        },
        last_match: None,
        last_checked: None,
    }
}

#[test]
fn test_alliance_default() {
    let alliance = Alliance::default();
    assert_eq!(alliance.auto(), 0);
    assert_eq!(alliance.teleop(), 0);
    assert_eq!(alliance.endgame(), 0);
    assert_eq!(alliance.penalties(), 0);
    assert_eq!(alliance.overall(), 0);
}

#[test]
fn test_alliance_auto_sum() {
    let team1 = create_test_team(1234, "Team A", 50, 0, 0, 0);
    let team2 = create_test_team(5678, "Team B", 75, 0, 0, 0);

    let alliance = Alliance::new(team1, team2, "Red".to_string());
    assert_eq!(alliance.auto(), 125); // 50 + 75
    assert_eq!(alliance.teleop(), 0);
    assert_eq!(alliance.endgame(), 0);
}

#[test]
fn test_alliance_with_real_scores() {
    let team1 = create_test_team(1234, "Team A", 100, 150, 40, 10);
    let team2 = create_test_team(5678, "Team B", 80, 120, 30, 5);

    let alliance = Alliance::new(team1, team2, "Blue".to_string());

    assert_eq!(alliance.auto(), 180); // 100 + 80
    assert_eq!(alliance.teleop(), 270); // 150 + 120
    assert_eq!(alliance.endgame(), 70); // 40 + 30
    assert_eq!(alliance.penalties(), 15); // 10 + 5
    assert_eq!(alliance.overall(), 505); // 180 + 270 + 70 - 15
}

#[test]
fn test_alliance_is_empty_when_zero_score() {
    let alliance = Alliance::default();
    assert!(alliance.is_empty());
}

#[test]
fn test_alliance_not_empty_with_scores() {
    let team1 = create_test_team(1234, "Team A", 50, 0, 0, 0);
    let team2 = create_test_team(5678, "Team B", 0, 0, 0, 0);
    let alliance = Alliance::new(team1, team2, "Red".to_string());

    assert!(!alliance.is_empty());
}

#[test]
fn test_alliance_names() {
    let team1 = create_test_team(1234, "Team A", 0, 0, 0, 0);
    let team2 = create_test_team(5678, "Team B", 0, 0, 0, 0);
    let alliance = Alliance::new(team1, team2, "Red".to_string());

    let names = alliance.names();
    assert_eq!(names.len(), 2);
    assert_eq!(names[0], "Team A");
    assert_eq!(names[1], "Team B");
}

#[test]
fn test_alliance_numbers() {
    let team1 = create_test_team(1234, "Team A", 0, 0, 0, 0);
    let team2 = create_test_team(5678, "Team B", 0, 0, 0, 0);
    let alliance = Alliance::new(team1, team2, "Blue".to_string());

    let numbers = alliance.numbers();
    assert_eq!(numbers.len(), 2);
    assert_eq!(numbers[0], 1234);
    assert_eq!(numbers[1], 5678);
}

#[test]
fn test_alliance_scoreboard_with_data() {
    let team1 = create_test_team(1234, "Team A", 100, 150, 40, 10);
    let team2 = create_test_team(5678, "Team B", 80, 120, 30, 5);
    let alliance = Alliance::new(team1, team2, "Red".to_string());

    let scoreboard = alliance.scoreboard();

    assert_eq!(scoreboard.get("Team 1"), Some(&1234));
    assert_eq!(scoreboard.get("Team 2"), Some(&5678));
    assert_eq!(scoreboard.get("Auto"), Some(&180));
    assert_eq!(scoreboard.get("Teleop"), Some(&270));
    assert_eq!(scoreboard.get("Endgame"), Some(&70));
    assert_eq!(scoreboard.get("Penalties"), Some(&15));
    assert_eq!(scoreboard.get("Overall"), Some(&505));
}

#[test]
fn test_alliance_scoreboard_structure() {
    let alliance = Alliance::default();
    let scoreboard = alliance.scoreboard();

    assert!(scoreboard.contains_key("Team 1"));
    assert!(scoreboard.contains_key("Team 2"));
    assert!(scoreboard.contains_key("Auto"));
    assert!(scoreboard.contains_key("Teleop"));
    assert!(scoreboard.contains_key("Endgame"));
    assert!(scoreboard.contains_key("Penalties"));
    assert!(scoreboard.contains_key("Overall"));
}

#[test]
fn test_alliance_scoreboard_default_values() {
    let alliance = Alliance::default();
    let scoreboard = alliance.scoreboard();

    assert_eq!(scoreboard.get("Auto"), Some(&0));
    assert_eq!(scoreboard.get("Teleop"), Some(&0));
    assert_eq!(scoreboard.get("Endgame"), Some(&0));
    assert_eq!(scoreboard.get("Penalties"), Some(&0));
    assert_eq!(scoreboard.get("Overall"), Some(&0));
}

#[test]
fn test_alliance_clone() {
    let alliance1 = Alliance::default();
    let alliance2 = alliance1.clone();

    assert_eq!(alliance1, alliance2);
}

#[test]
fn test_alliance_debug_format() {
    let alliance = Alliance::default();
    let debug_str = format!("{:?}", alliance);
    assert!(debug_str.contains("Alliance"));
}
