use cache::season::{Database, Season};

#[test]
fn test_season_default() {
    let season = Season::default();
    let debug_str = format!("{:?}", season);
    assert!(debug_str.contains("Season"));
}

#[test]
fn test_season_clone() {
    let season1 = Season::default();
    let season2 = season1.clone();
    assert_eq!(season1, season2);
}

#[test]
fn test_season_partial_eq() {
    let season1 = Season::default();
    let season2 = Season::default();
    assert_eq!(season1, season2);
}

#[test]
fn test_season_debug_format() {
    let season = Season::default();
    let debug_str = format!("{:?}", season);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_database_default() {
    let database = Database::default();
    let debug_str = format!("{:?}", database);
    assert!(debug_str.contains("Database"));
}

#[test]
fn test_database_clone() {
    let db1 = Database::default();
    let db2 = db1.clone();
    assert_eq!(db1, db2);
}

#[test]
fn test_database_partial_eq() {
    let db1 = Database::default();
    let db2 = Database::default();
    assert_eq!(db1, db2);
}

#[test]
fn test_database_debug_format() {
    let database = Database::default();
    let debug_str = format!("{:?}", database);
    assert!(!debug_str.is_empty());
}
