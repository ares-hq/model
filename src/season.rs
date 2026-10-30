/// A season starts on the first of this month.
pub const ROLLOVER_MONTH: u32 = 8;

/// Rolls over in August: May 2027 is the 2026 season.
pub fn current_season(year: i32, month: u32) -> i32 {
    if month < ROLLOVER_MONTH {
        year - 1
    } else {
        year
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rolls_over_in_august() {
        assert_eq!(current_season(2026, 7), 2025);
        assert_eq!(current_season(2026, 8), 2026);
        assert_eq!(current_season(2026, 1), 2025);
        assert_eq!(current_season(2026, 12), 2026);
    }
}
