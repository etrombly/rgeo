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

use kdtree::KdTree;
use kdtree::distance::squared_euclidean;
use bincode::deserialize;

use record::Record;

/// parsed country
pub mod record;
/// goenamedb entry
pub mod country;

lazy_static! {
    static ref GEO: KdTree<f64, Record, [f64; 2]> = {
        let encoded = include_bytes!("data/output.bin");
        deserialize(&encoded[..]).unwrap()
    };
}

/// search for closest record to target lat/long returns Option<distance, record>
pub fn search<'a>(lat: f64, lon: f64) -> Option<(f64, &'a record::Record)> {
    match GEO.nearest(&[lat, lon], 1, &squared_euclidean) {
        Ok(x) => if x.is_empty() {
            None
        } else {
            Some(x[0])
        },
        Err(_) => None,
    }
}
