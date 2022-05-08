#![deny(missing_docs)]
//!# rgeo
//!
//!Rust library for reverse geocoding. Uses data from the geonames database. http://www.geonames.org/
//!
//!Usage:
//!```
//!extern crate rgeo;
//!use rgeo::search;
//!
//!search(44.353339_f32, -72.740231_f32);
//!// Some((0.0001186200820000013, Record { name: "Village of Waterbury", latitude: 44.34279, longitude: -72.74294, country: "US" }))
//!```

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;

use bincode::deserialize;
use kiddo::KdTree;
use num_traits::{cast::FromPrimitive, float::Float};

use record::{Record, Nvec};

/// goenamedb entry
pub mod country;
/// parsed country
pub mod record;

lazy_static! {
    static ref GEO: KdTree<f32, Record, 3> = {
        let encoded = include_bytes!("data/output.bin");
        deserialize(&encoded[..]).unwrap()
    };
}

/// calculate the dot product of two nvecs
pub fn dot<T: Float + FromPrimitive, const K: usize>(left: &[T; K], right: &[T; K]) -> T  {
    let x = left[0] * right[0];
    let y = left[1] * right[1];
    let z = left[2] * right[2];
    x + y + z
}

/// calculate the great circle distance between two nvecs
pub fn distance<T: Float + FromPrimitive, const K: usize>(left: &[T; K], right: &[T; K]) -> T {
    dot(left, right).acos() * T::from_f32(3_958.8_f32).unwrap()
}

/// search for closest record to target lat/long returns Option<distance, record>
pub fn search<'a>(lat: f32, lon: f32) -> Option<(f32, &'a record::Record)> {
    let nvec = Nvec::from_lat_long(lat, lon);
    match GEO.nearest_one(&[nvec.x, nvec.y, nvec.z], &distance) {
        Ok(x) => {
            Some(x)
        }
        Err(_) => None,
    }
}

/// search for closest records to target lat/long returns Option<distance, record>
pub fn nearest<'a>(lat: f32, lon: f32, number: usize) -> Option<Vec<(f32, &'a record::Record)>> {
    let nvec = Nvec::from_lat_long(lat, lon);
    match GEO.nearest(&[nvec.x, nvec.y, nvec.z], number, &distance) {
        Ok(x) => {
            if x.is_empty() {
                None
            } else {
                Some(x)
            }
        }
        Err(_) => None,
    }
}
