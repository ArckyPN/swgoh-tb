use serde::Deserialize;

use crate::api::teams::{omicron::Omicrons, video::Video};

#[derive(Debug, Deserialize, Clone, Hash)]
pub struct Mission {
    /// the ID of a mission
    ///
    /// used by in-game orders to
    /// CTRL + F find the correct team
    /// or using search
    pub id: String,
    /// Name of the Mission
    pub name: String,
    /// list of unit IDs which form the
    /// team used for this mission
    pub team: Vec<String>,
    /// additional note giving information
    /// about this mission
    ///
    /// each element is one paragraph
    pub note: Vec<String>,
    /// the relic requirement of this
    /// mission
    ///
    /// fleet mission only require 7 stars
    pub relic: Option<u8>,
    /// omicrons required for this mission
    pub omicrons: Option<Vec<Omicrons>>,
    /// videos showcasing this team
    pub videos: Option<Vec<Video>>,
    /// modding recommendation
    pub modding: Option<String>,
}
