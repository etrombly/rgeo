#[derive(Debug, Deserialize, Serialize)]
/// parsed country
pub struct Record {
    /// location name
    pub name: String,
    /// latitude
    pub latitude: f32,
    /// longitude
    pub longitude: f32,
    #[serde(rename = "country code")]
    /// country code from geonamedb
    pub country: String,
}
