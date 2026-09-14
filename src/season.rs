const AUGUST: u32 = 8;

/// The competition season a calendar month falls in; seasons roll over in August.
/// Example: May 2027 (`2027, 5`) would result in 2026 (the 2026-2027 season)
pub fn current_season(year: i32, month: u32) -> i32 {
    if month < AUGUST { year - 1 } else { year }
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
