use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Level {
    Practice,
    Qualification,
    Playoff,
    /// Anything the API sends that this crate does not know.
    #[default]
    #[serde(other, rename = "Unknown")]
    Unknown,
}

impl Level {
    fn as_str(self) -> &'static str {
        match self {
            Self::Practice => "PRACTICE",
            Self::Qualification => "QUALIFICATION",
            Self::Playoff => "PLAYOFF",
            Self::Unknown => "Unknown",
        }
    }
}

impl std::fmt::Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(s: &str) -> Level {
        serde_json::from_value(serde_json::json!(s)).unwrap()
    }

    #[test]
    fn round_trips_the_api_string() {
        for s in ["PRACTICE", "QUALIFICATION", "PLAYOFF", "Unknown"] {
            assert_eq!(parse(s).as_str(), s);
            assert_eq!(
                serde_json::to_value(parse(s)).unwrap(),
                serde_json::json!(s)
            );
        }
    }

    #[test]
    fn unknown_strings_fall_through() {
        assert_eq!(parse("SOMETHING_NEW"), Level::Unknown);
        assert_eq!(parse(""), Level::Unknown);
    }
}
