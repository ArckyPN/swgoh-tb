use serde::Deserialize;

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
    pub note: String,
    // TODO add relic requirement
}
