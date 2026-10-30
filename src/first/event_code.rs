use serde::{Deserialize, Deserializer, Serialize};
use std::borrow::Borrow;
use std::fmt;
use std::str::FromStr;

/// Uppercase ASCII alphanumerics, e.g. `USAZTUQ`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize)]
#[serde(transparent)]
pub struct EventCode(String);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidEventCode;

impl fmt::Display for InvalidEventCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("event codes are letters and digits only")
    }
}

impl std::error::Error for InvalidEventCode {}

impl EventCode {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl FromStr for EventCode {
    type Err = InvalidEventCode;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        let code = raw.trim();
        if code.is_empty() || !code.bytes().all(|b| b.is_ascii_alphanumeric()) {
            return Err(InvalidEventCode);
        }
        Ok(Self(code.to_ascii_uppercase()))
    }
}

impl<'de> Deserialize<'de> for EventCode {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        String::deserialize(d)?
            .parse()
            .map_err(serde::de::Error::custom)
    }
}

impl fmt::Display for EventCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl AsRef<str> for EventCode {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Borrow<str> for EventCode {
    fn borrow(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_is_trimmed_and_uppercased() {
        assert_eq!(
            "  usaztuq ".parse::<EventCode>().unwrap().as_str(),
            "USAZTUQ"
        );
    }

    #[test]
    fn anything_but_letters_and_digits_is_rejected() {
        for bad in ["", "   ", "US TX", "US-TX", "ÜS"] {
            assert_eq!(bad.parse::<EventCode>(), Err(InvalidEventCode), "{bad:?}");
        }
    }

    #[test]
    fn deserializes_through_validation() {
        let code: EventCode = serde_json::from_value(serde_json::json!("usaztuq")).unwrap();
        assert_eq!(code.as_str(), "USAZTUQ");
        assert!(serde_json::from_value::<EventCode>(serde_json::json!("a b")).is_err());
    }
}
