#[derive(Debug, Deserialize, Serialize)]
/// parsed country
pub struct Record {
    /// location name
    pub name: String,
    /// unit vector
    pub nvec: Nvec,
    #[serde(rename = "country code")]
    /// country code from geonamedb
    pub country: String,
}

/// n vector representation of location
#[derive(Debug, Serialize, Deserialize)]
pub struct Nvec {
    /// x coordinate
    pub x: f32,
    /// y coordinate
    pub y: f32,
    /// z coordinate
    pub z: f32,
}

impl Nvec {
    /// convert from lat long to n vector
    pub fn from_lat_long(lat: f32, long: f32) -> Nvec {
        let x = lat.to_radians().cos() * long.to_radians().cos();
        let y = lat.to_radians().cos() * long.to_radians().sin();
        let z = lat.to_radians().sin();
        Nvec{x,y,z}
    }
}