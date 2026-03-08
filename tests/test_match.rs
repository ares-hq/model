use cache::alliance::Alliance;
use cache::r#match::Match;

#[test]
fn test_match_default() {
    let match_game = Match::default();
    assert_eq!(match_game.red.overall(), 0);
    assert_eq!(match_game.blue.overall(), 0);
}

#[test]
fn test_match_winner_tie() {
    let match_game = Match::default();
    assert_eq!(match_game.winner(), None);
}

#[test]
fn test_match_is_tie_default() {
    let match_game = Match::default();
    assert!(match_game.is_tie());
}

#[test]
fn test_match_is_tie_equal_scores() {
    let match_game = Match {
        red: Alliance::default(),
        blue: Alliance::default(),
    };
    assert!(match_game.is_tie());
}

#[test]
fn test_match_clone() {
    let match1 = Match::default();
    let match2 = match1.clone();

    assert_eq!(match1, match2);
}

#[test]
fn test_match_debug_format() {
    let match_game = Match::default();
    let debug_str = format!("{:?}", match_game);
    assert!(debug_str.contains("Match"));
}

#[test]
fn test_match_partial_eq() {
    let match1 = Match::default();
    let match2 = Match::default();

    assert_eq!(match1, match2);
}

#[test]
fn test_match_winner_returns_option() {
    let match_game = Match::default();
    let winner = match_game.winner();

    // winner should be Option<&str>
    assert!(winner.is_none() || winner == Some("Red") || winner == Some("Blue"));
}
