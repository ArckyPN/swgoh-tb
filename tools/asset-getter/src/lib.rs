mod crawler;

use anyhow::Result;
pub use crawler::*;
use thirtyfour::prelude::*;

use std::fmt::Display;

pub const BASE_URL: &str = "https://swgoh.gg";
pub const BASE_OUTPUT: &str = "../../assets";

#[derive(Debug, Copy, Clone)]
pub enum Type {
    Character,
    Ship,
}

impl Type {
    pub fn portrait(&self) -> String {
        match self {
            Self::Character => "character".to_owned(),
            Self::Ship => "ship".to_owned(),
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Character => f.write_str("characters"),
            Self::Ship => f.write_str("ships"),
        }
    }
}

pub async fn class_inner_html(element: &WebElement, class: &str) -> Result<String> {
    Ok(element
        .query(By::ClassName(class))
        .first()
        .await?
        .inner_html()
        .await?)
}

pub fn enclosed_substring(s: &str, from: &str, to: &str) -> String {
    let f = s.find(from).map(|f| f + 1).unwrap_or(0);
    let t = s.find(to).unwrap_or(s.len());

    s[f..t].to_owned()
}

pub fn clean_name(s: &str) -> String {
    s.trim_matches('\n').trim().to_owned().replace("&amp;", "&")
}
