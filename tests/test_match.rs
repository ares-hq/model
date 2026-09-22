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
    assert_eq!(m.totals(), (0.0, 0.0));
    assert_eq!(m.winner(), None);
    assert!(m.red.is_empty() && m.blue.is_empty());
}

#[test]
fn winner_is_decided_by_measured_score() {
    assert_eq!(played(120.0, 95.0).winner(), Some(Winner::Red));
    assert_eq!(played(95.0, 120.0).winner(), Some(Winner::Blue));
}

#[test]
fn equal_scores_are_a_tie() {
    assert_eq!(played(100.0, 100.0).winner(), None);
}

#[test]
fn summed_opr_within_rounding_distance_is_a_tie() {
    assert_eq!(played(100.0, 100.001).winner(), None);
    assert_eq!(played(100.0, 100.5).winner(), Some(Winner::Blue));
}

#[test]
fn fouls_are_awarded_to_the_opponent() {
    let m = Match::new(
        Alliance {
            teams: [1, 2],
            auto: 100.0,
            penalties: 30.0,
            ..Default::default()
        },
        Alliance {
            teams: [3, 4],
            auto: 90.0,
            penalties: 5.0,
            ..Default::default()
        },
    );

    // Red scored 100 and gave away 30; blue scored 90 and gave away 5.
    assert_eq!(m.totals(), (105.0, 120.0));
    assert_eq!(m.winner(), Some(Winner::Blue));
}

#[test]
fn a_foul_can_decide_the_match() {
    let clean = Match::new(
        Alliance {
            auto: 100.0,
            ..Default::default()
        },
        Alliance {
            auto: 95.0,
            ..Default::default()
        },
    );
    assert_eq!(clean.winner(), Some(Winner::Red));

    let mut penalised = clean.clone();
    penalised.red.penalties = 10.0;
    assert_eq!(penalised.totals(), (100.0, 105.0));
    assert_eq!(penalised.winner(), Some(Winner::Blue));
}

#[test]
fn net_points_ranks_alliances_the_same_way_totals_do() {
    // `net` keeps its own fouls; `totals` hands them over. Different numbers,
    // same ordering — the pipeline relies on that.
    let m = Match::new(
        Alliance {
            auto: 100.0,
            penalties: 30.0,
            ..Default::default()
        },
        Alliance {
            auto: 90.0,
            penalties: 5.0,
            ..Default::default()
        },
    );
    let (red_total, blue_total) = m.totals();
    assert_eq!(
        red_total < blue_total,
        m.red.net_points() < m.blue.net_points()
    );
}

#[test]
fn winner_renders_as_its_colour() {
    assert_eq!(Winner::Red.as_str(), "Red");
    assert_eq!(Winner::Blue.as_str(), "Blue");
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
