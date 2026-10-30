pub fn season(year: i32) -> String {
    format!("season_{year}")
}

pub fn matches(year: i32) -> String {
    format!("matches_{year}")
}

pub const FAVORITES: &str = "favorites";
pub const EVENT_SUBSCRIPTIONS: &str = "event_subscriptions";
pub const POSTED_MATCHES: &str = "posted_matches";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_match_the_deployed_schema() {
        assert_eq!(season(2025), "season_2025");
        assert_eq!(matches(2025), "matches_2025");
    }
}
