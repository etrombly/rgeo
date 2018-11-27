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
//!search(44.353339_f64, -72.740231_f64);
//!// Some((0.0001186200820000013, Record { name: "Village of Waterbury", latitude: 44.34279, longitude: -72.74294, country: "US" }))
//!```

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate kdtree;
#[macro_use]
extern crate lazy_static;

use bincode::deserialize;
use kdtree::{
    distance::squared_euclidean,
    KdTree
};

use crate::record::Record;

/// goenamedb entry
pub mod country;
/// parsed country
pub mod record;

lazy_static! {
    static ref GEO: KdTree<f32, Record, [f32; 2]> = {
        let encoded = include_bytes!("data/output.bin");
        deserialize(&encoded[..]).unwrap()
    };
}

/// search for closest record to target lat/long returns Option<distance, record>
pub fn search<'a>(lat: f32, lon: f32) -> Option<(f32, &'a record::Record)> {
    match GEO.nearest(&[lat, lon], 1, &squared_euclidean) {
        Ok(x) => {
            if x.is_empty() {
                None
            } else {
                Some(x[0])
            }
        }
        Err(_) => None,
    }
}

/// search for closest records to target lat/long returns Option<distance, record>
pub fn nearest<'a>(lat: f32, lon: f32, number: usize) -> Option<Vec<(f32, &'a record::Record)>> {
    match GEO.nearest(&[lat, lon], number, &squared_euclidean) {
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
