use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Hash)]
pub struct Video {
    /// source of the video, usually the
    /// content creator
    #[serde(rename = "src")]
    pub source: String,
    /// href to the video
    pub url: String,
}
