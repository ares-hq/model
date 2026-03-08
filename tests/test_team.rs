use std::time::SystemTime;
use cache::scoring::{Auto, Teleop, Endgame, Penalties};
use cache::team::Team;

#[test]
fn test_attributes() {
    let team = Team {
        number: 1234,
        name: "Test Team".to_string(),
        sponsors: "Test Sponsor".to_string(),
        location: "Test Location".to_string(),
        auto: Auto { score: 100, rank: 1 },
        teleop: Teleop { score: 200, rank: 2 },
        endgame: Endgame { score: 50, rank: 3 },
        penalties: Penalties { score: 10, rank: 4 },
        last_match: Some(SystemTime::now()),
        last_checked: Some(SystemTime::now()),
    };

    let team2 = Team {
        number: 5678,
        name: "Another Team".to_string(),
        sponsors: String::new(),
        location: String::new(),
        auto: Auto::default(),
        teleop: Teleop::default(),
        endgame: Endgame::default(),
        penalties: Penalties::default(),
        last_match: Some(SystemTime::now()),
        last_checked: Some(SystemTime::now()),
    };

    assert_eq!(team.number, 1234);
    assert_eq!(team.name, "Test Team");
    assert_eq!(team.sponsors, "Test Sponsor");
    assert_eq!(team.location, "Test Location");
    assert_eq!(team.auto.score, 100);
    assert_eq!(team.auto.rank, 1);
    assert_eq!(team.teleop.score, 200);
    assert_eq!(team.teleop.rank, 2);
    assert_eq!(team.endgame.score, 50);
    assert_eq!(team.endgame.rank, 3);
    assert_eq!(team.penalties.score, 10);
    assert_eq!(team.penalties.rank, 4);
    assert!(team.last_match.is_some());
    assert!(team.last_checked.is_some());

    assert_eq!(team2.number, 5678);
    assert_eq!(team2.name, "Another Team");
    assert_eq!(team2.auto.score, 0);
    assert_eq!(team2.teleop.score, 0);
    assert_eq!(team2.endgame.score, 0);
    assert_eq!(team2.penalties.score, 0);
}
