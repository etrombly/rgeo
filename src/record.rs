#[derive(Debug, Deserialize, Serialize)]
/// parsed country
pub struct Record {
    /// location name
    pub name: String,
    /// latitude
    pub latitude: f64,
    /// longitude
    pub longitude: f64,
    #[serde(rename="country code")]
    /// country code from geonamedb
    pub country: String,
}