#[derive(Debug, Deserialize, Serialize)]
/// goenamedb entry
pub struct Country {
    /// id from geonamedb
    pub geonameid: u32,
    /// name
    pub name: String,
    #[serde(rename = "country code")]
    /// country code from geonamedb
    pub country: String,
}
