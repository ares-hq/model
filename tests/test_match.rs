use model::prelude::{Alliance, Match, Winner};

fn played(red: f64, blue: f64) -> Match {
    Match::new(
        Alliance {
            teams: [1, 2],
            auto: red,
            ..Default::default()
        },
        Alliance {
            teams: [3, 4],
            auto: blue,
            ..Default::default()
        },
    )
}

#[test]
fn default_match_is_unplayed_and_tied() {
    let m = Match::default();
    assert_eq!(m.red.overall(), 0.0);
    assert_eq!(m.blue.overall(), 0.0);
    assert_eq!(m.winner(), None);
    assert!(m.is_tie());
    assert!(!m.is_complete());
}

#[test]
fn winner_is_decided_by_measured_score() {
    assert_eq!(played(120.0, 95.0).winner(), Some(Winner::Red));
    assert_eq!(played(95.0, 120.0).winner(), Some(Winner::Blue));
}

#[test]
fn equal_scores_are_a_tie() {
    let m = played(100.0, 100.0);
    assert_eq!(m.winner(), None);
    assert!(m.is_tie());
    assert_eq!(m.score_difference(), 0.0);
}

#[test]
fn score_difference_is_unsigned() {
    assert_eq!(played(120.0, 95.0).score_difference(), 25.0);
    assert_eq!(played(95.0, 120.0).score_difference(), 25.0);
}

#[test]
fn total_score_sums_both_alliances() {
    assert_eq!(played(120.0, 95.0).total_score(), 215.0);
}

#[test]
fn a_match_with_any_score_is_complete() {
    assert!(played(1.0, 0.0).is_complete());
    assert!(played(0.0, 1.0).is_complete());
}

#[test]
fn alliances_yields_red_then_blue() {
    let m = played(120.0, 95.0);
    let [red, blue] = m.alliances();
    assert_eq!(red.teams, [1, 2]);
    assert_eq!(blue.teams, [3, 4]);
}

#[test]
fn match_clones_and_compares() {
    let m = played(120.0, 95.0);
    assert_eq!(m, m.clone());
    assert!(format!("{m:?}").contains("Match"));
}
