use std::fmt::Display;

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Hash)]
pub struct Omicrons {
    /// Unit ID which these Omicrons is applied to
    pub unit: String,
    /// The applied Omicrons
    pub omis: Vec<Omicron>,
}

#[derive(Debug, Clone, Hash)]
pub enum Omicron {
    Basic,
    Special(u8),
    Lead,
    Unique(u8),
}

impl Display for Omicron {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Basic => f.write_str("Basic"),
            Self::Lead => f.write_str("Lead"),
            Self::Special(s) => write!(f, "Special {s}"),
            Self::Unique(u) => write!(f, "Unique {u}"),
        }
    }
}

impl<'de> Deserialize<'de> for Omicron {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let variant = String::deserialize(deserializer)?;

        match variant.to_ascii_lowercase().as_str() {
            "b" => Ok(Self::Basic),
            "l" => Ok(Self::Lead),
            s if s.starts_with('s') => {
                Ok(Self::Special(s.trim_start_matches('s').parse().map_err(
                    |_err| serde::de::Error::custom("failed to parse special num"),
                )?))
            }
            u if u.starts_with('u') => {
                Ok(Self::Unique(u.trim_start_matches('u').parse().map_err(
                    |_err| serde::de::Error::custom("failed to parse unique num"),
                )?))
            }
            _ => Err(serde::de::Error::custom("invalid ability variant")),
        }
    }
}
