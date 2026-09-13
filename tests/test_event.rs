use model::prelude::{Alliance, Event, Match};

fn qual(red: [u32; 2], blue: [u32; 2]) -> Match {
    Match::new(Alliance::new(red), Alliance::new(blue))
}

#[test]
fn default_event_is_empty() {
    let event = Event::default();
    assert_eq!(event.code, "");
    assert!(event.is_empty());
    assert_eq!(event.match_count(), 0);
    assert!(event.teams().is_empty());
}

#[test]
fn new_sets_the_code() {
    assert_eq!(Event::new("TXHOU").code, "TXHOU");
}

#[test]
fn matches_keep_the_order_they_were_added() {
    let mut event = Event::new("TXHOU");
    event.add_match(qual([1, 2], [3, 4]));
    event.add_match(qual([5, 6], [7, 8]));

    assert_eq!(event.match_count(), 2);
    assert_eq!(event.matches[0].red.teams, [1, 2]);
    assert_eq!(event.matches[1].red.teams, [5, 6]);
}

#[test]
fn teams_are_listed_in_order_of_first_appearance() {
    let mut event = Event::new("TXHOU");
    event.add_match(qual([1, 2], [3, 4]));
    event.add_match(qual([5, 6], [7, 8]));

    assert_eq!(event.teams(), vec![1, 2, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn teams_are_deduplicated_across_matches() {
    let mut event = Event::new("TXHOU");
    event.add_match(qual([1, 2], [3, 4]));
    // Same eight teams, reshuffled into new alliances.
    event.add_match(qual([1, 3], [2, 4]));

    assert_eq!(event.teams(), vec![1, 2, 3, 4]);
}

#[test]
fn team_order_is_stable_so_opr_columns_are_reproducible() {
    let mut event = Event::new("TXHOU");
    for _ in 0..2 {
        event.add_match(qual([9, 7], [4, 1]));
    }
    // First appearance order, not sorted order.
    assert_eq!(event.teams(), vec![9, 7, 4, 1]);
}

#[test]
fn realistic_event_shape() {
    let mut event = Event::new("TXHOU");
    for i in 0..30 {
        let base = (i % 5) * 4 + 1000;
        event.add_match(qual([base, base + 1], [base + 2, base + 3]));
    }

    assert_eq!(event.match_count(), 30);
    assert_eq!(event.teams().len(), 20);
}

#[test]
fn event_clones_and_compares() {
    let mut event = Event::new("TXHOU");
    event.add_match(qual([1, 2], [3, 4]));

    assert_eq!(event, event.clone());
    assert!(format!("{event:?}").contains("Event"));
}

#[test]
fn empty_team_slots_are_not_counted_as_a_team() {
    let mut event = Event::new("TXHOU");
    // Blue ran a match a team short.
    event.add_match(qual([1, 2], [3, 0]));

    assert_eq!(event.teams(), vec![1, 2, 3]);
}
