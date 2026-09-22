use model::prelude::Team;
use serde_json::json;

/// Stand-in for the pipeline's own build-up: set the phases, then derive `overall`.
fn scored(number: u32, name: &str, auto: f64, teleop: f64, endgame: f64, penalties: f64) -> Team {
    let mut team = Team {
        auto,
        teleop,
        endgame,
        penalties,
        ..Team::new(number, name.into())
    };
    team.recompute_overall();
    team
}

#[test]
fn new_leaves_everything_else_default() {
    let team = Team::new(1234, "Alpha".into());
    assert_eq!(team.number, 1234);
    assert_eq!(team.auto, 0.0);
    assert_eq!(team.auto_rank, None);
    assert_eq!(team.overall, 0.0);
    assert!(team.events_attended.is_empty());
}

#[test]
fn overall_is_auto_plus_teleop_only() {
    let team = scored(1, "A", 10.0, 20.0, 5.0, 3.0);
    assert_eq!(team.overall, 30.0, "endgame and penalties rank separately");
}

#[test]
fn scores_hold_negative_and_fractional_opr() {
    let team = scored(1, "A", -1.8366, 45.3513, 3.25, 62.6598);
    assert_eq!(team.auto, -1.8366);
    assert!((team.overall - 43.5147).abs() < 1e-9);
}

#[test]
fn overall_goes_negative_instead_of_underflowing() {
    let team = scored(1, "A", -10.0, 1.0, 1.0, 500.0);
    assert_eq!(team.overall, -9.0);
}

#[test]
fn recompute_overall_tracks_edits() {
    let mut team = scored(1, "A", 10.0, 20.0, 0.0, 0.0);
    team.auto = 99.0;
    team.recompute_overall();
    assert_eq!(team.overall, 119.0);
}

#[test]
fn recompute_overall_leaves_ranks_alone() {
    let mut team = scored(1, "A", 10.0, 20.0, 0.0, 0.0);
    team.overall_rank = Some(7);
    team.recompute_overall();
    assert_eq!(team.overall_rank, Some(7));
}

#[test]
fn add_events_sorts_and_deduplicates() {
    let mut team = Team::new(1, "A".into());
    team.add_events(["TXHOU".into(), "ARCMP".into()]);
    team.add_events(["TXHOU".into(), "USTXCMP".into()]);
    assert_eq!(team.events_attended, ["ARCMP", "TXHOU", "USTXCMP"]);
}

#[test]
fn serializes_to_the_season_table_columns() {
    let mut team = scored(1234, "Alpha", -1.5, 45.25, 3.0, 62.0);
    team.sponsors = "Acme".into();
    team.location = "Houston, TX, USA".into();
    team.founded = Some(2011);
    team.website = Some("https://example.com".into());
    team.auto_rank = Some(42);
    team.tele_rank = Some(7);
    team.endgame_rank = Some(9);
    team.penalty_rank = Some(900);
    team.overall_rank = Some(11);
    team.add_events(["TXHOU".into()]);
    team.profile_update = Some("2026-01-01T00:00:00.000".into());

    let json = serde_json::to_value(&team).unwrap();
    let mut keys: Vec<&str> = json
        .as_object()
        .unwrap()
        .keys()
        .map(String::as_str)
        .collect();
    keys.sort_unstable();

    assert_eq!(
        keys,
        [
            "autoOPR",
            "autoRank",
            "endgameOPR",
            "endgameRank",
            "eventsAttended",
            "founded",
            "location",
            "overallOPR",
            "overallRank",
            "penalties",
            "penaltyRank",
            "profileUpdate",
            "sponsors",
            "teamName",
            "teamNumber",
            "teleOPR",
            "teleRank",
            "website",
        ]
    );
    assert_eq!(json["teamNumber"], 1234);
    assert_eq!(json["autoOPR"], -1.5);
    assert_eq!(json["overallOPR"], 43.75);
    assert_eq!(json["eventsAttended"], json!(["TXHOU"]));
}

#[test]
fn round_trips_through_json() {
    let mut team = scored(1234, "Alpha", -1.5, 45.25, 3.0, 62.0);
    team.auto_rank = Some(42);
    team.add_events(["TXHOU".into()]);

    let back: Team = serde_json::from_value(serde_json::to_value(&team).unwrap()).unwrap();
    assert_eq!(back, team);
}

#[test]
fn missing_columns_fall_back_to_defaults() {
    let team: Team = serde_json::from_value(json!({ "teamNumber": 99 })).unwrap();
    assert_eq!(team.number, 99);
    assert_eq!(team.name, "");
    assert_eq!(team.auto, 0.0);
    assert_eq!(team.auto_rank, None);
    assert!(team.events_attended.is_empty());
}

#[test]
fn null_ranks_read_back_as_unranked() {
    let team: Team = serde_json::from_value(json!({ "autoRank": null })).unwrap();
    assert_eq!(team.auto_rank, None);
}

#[test]
fn events_deserialize_from_a_jsonb_array() {
    let team: Team =
        serde_json::from_value(json!({ "eventsAttended": ["TXHOU", "ARCMP"] })).unwrap();
    assert_eq!(team.events_attended, ["TXHOU", "ARCMP"]);
}

/// A real row from `season_2025`, including the four columns the pipeline
/// never populates.
#[test]
fn deserializes_a_live_season_row() {
    let row = json!({
        "teamNumber": 26964,
        "teamName": "Fúria do Norte ",
        "sponsors": "Sesi Marabá",
        "location": "Marabá , PA, Brazil",
        "teleOPR": 48.2333333333333,
        "autoOPR": 10.7333333333333,
        "endgameOPR": 1.66666666666667,
        "overallOPR": 58.9666666666667,
        "penalties": -1.66666666666667,
        "teleRank": 1037,
        "autoRank": 2417,
        "endgameRank": 7037,
        "overallRank": 1368,
        "penaltyRank": 1382,
        "profileUpdate": "2026-02-07T17:27:15.192+00:00",
        "founded": 2024,
        "website": "https://furia-do-norte7.webnode.page/",
        "eventsAttended": ["BRANQ"],
        // Columns the pipeline does not write.
        "eventDate": null,
        "teamLogo": "",
        "achievements": null,
        "averagePlace": 0,
    });

    let team: Team = serde_json::from_value(row).unwrap();

    assert_eq!(team.number, 26964);
    assert_eq!(team.name, "Fúria do Norte ");
    assert_eq!(team.auto_rank, Some(2417));
    assert_eq!(team.penalties, -1.66666666666667);
    assert_eq!(team.events_attended, ["BRANQ"]);
    assert_eq!(team.founded, Some(2024));
}

/// The upsert payload must not name columns the table does not have.
#[test]
fn serialized_columns_all_exist_in_the_season_table() {
    const COLUMNS: [&str; 22] = [
        "teamNumber",
        "teamName",
        "sponsors",
        "location",
        "autoOPR",
        "teleOPR",
        "endgameOPR",
        "overallOPR",
        "penalties",
        "autoRank",
        "teleRank",
        "endgameRank",
        "overallRank",
        "penaltyRank",
        "profileUpdate",
        "founded",
        "website",
        "eventsAttended",
        "eventDate",
        "teamLogo",
        "achievements",
        "averagePlace",
    ];

    let json = serde_json::to_value(Team::new(1, "A".into())).unwrap();
    for key in json.as_object().unwrap().keys() {
        assert!(COLUMNS.contains(&key.as_str()), "no such column: {key}");
    }
}
