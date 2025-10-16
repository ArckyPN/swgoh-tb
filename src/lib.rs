#![warn(clippy::all, rust_2018_idioms)]

mod api;
mod app;
use std::{fmt::Display, str::FromStr};

pub use api::*;
#[cfg(target_arch = "wasm32")]
pub use app::App;

#[derive(PartialEq, Eq)]
pub enum Tab {
    Info,
    Phase(usize),
}

impl FromStr for Tab {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_ascii_lowercase().as_str() {
            "info" => Self::Info,
            x => Self::Phase(x.parse().unwrap_or(1)),
        })
    }
}

impl Display for Tab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Info => write!(f, "info"),
            Self::Phase(x) => write!(f, "{x}"),
        }
    }
}

impl Default for Tab {
    fn default() -> Self {
        Self::Phase(1)
    }
}

pub struct Resolution {
    pub height: f32,
    pub width: f32,
}
