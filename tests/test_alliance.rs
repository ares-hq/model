use model::prelude::Alliance;

/// A realistic qualification alliance: scores measured for the pair, not per team.
fn scored() -> Alliance {
    Alliance {
        teams: [1234, 5678],
        auto: 180.0,
        teleop: 270.0,
        endgame: 70.0,
        penalties: 15.0,
    }
}

#[test]
fn default_alliance_scores_nothing() {
    let alliance = Alliance::default();
    assert_eq!(alliance.teams, [0, 0]);
    assert_eq!(alliance.overall(), 0.0);
    assert!(alliance.is_empty());
}

#[test]
fn new_records_teams_without_scores() {
    let alliance = Alliance::new([1234, 5678]);
    assert_eq!(alliance.teams, [1234, 5678]);
    assert_eq!(alliance.overall(), 0.0);
}

#[test]
fn overall_deducts_penalties_from_every_phase() {
    // 180 + 270 + 70 - 15
    assert_eq!(scored().overall(), 505.0);
}

#[test]
fn scores_are_the_alliances_own_not_a_sum_of_teams() {
    // The whole point of the pre-solve view: these came off the field as a
    // pair, and no per-team split exists yet.
    let alliance = scored();
    assert_eq!(alliance.auto, 180.0);
    assert_eq!(alliance.teleop, 270.0);
    assert_eq!(alliance.endgame, 70.0);
    assert_eq!(alliance.penalties, 15.0);
}

#[test]
fn scoring_in_any_phase_makes_it_non_empty() {
    let alliance = Alliance {
        teams: [1, 2],
        auto: 50.0,
        ..Default::default()
    };
    assert!(!alliance.is_empty());
}

#[test]
fn penalties_alone_leave_a_negative_score() {
    let alliance = Alliance {
        teams: [1, 2],
        penalties: 20.0,
        ..Default::default()
    };
    assert_eq!(alliance.overall(), -20.0);
    assert!(!alliance.is_empty());
}

#[test]
fn has_team_matches_either_slot() {
    let alliance = scored();
    assert!(alliance.has_team(1234));
    assert!(alliance.has_team(5678));
    assert!(!alliance.has_team(9999));
}

#[test]
fn alliance_clones_and_compares() {
    let alliance = scored();
    assert_eq!(alliance, alliance.clone());
    assert!(format!("{alliance:?}").contains("Alliance"));
}

#[test]
fn zero_marks_an_empty_slot() {
    // A team the field reported as absent leaves its slot unfilled.
    let alliance = Alliance {
        teams: [1234, 0],
        auto: 10.0,
        teleop: 20.0,
        ..Default::default()
    };

    assert!(alliance.has_team(1234));
    assert!(!alliance.has_team(0));
    assert_eq!(alliance.present().collect::<Vec<_>>(), vec![1234]);
}

#[test]
fn present_yields_both_teams_when_the_alliance_is_full() {
    assert_eq!(scored().present().collect::<Vec<_>>(), vec![1234, 5678]);
}
