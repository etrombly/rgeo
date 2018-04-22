#[derive(Debug, Deserialize, Serialize)]
pub struct Country {
    pub geonameid: u32,
    pub name: String,
    #[serde(rename="country code")]
    pub country: String,
}