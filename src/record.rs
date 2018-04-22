#[derive(Debug, Deserialize, Serialize)]
pub struct Record {
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    #[serde(rename="country code")]
    pub country: String,
}