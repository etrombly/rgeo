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

pub mod record;
pub mod country;


lazy_static! {
    static ref GEO: KdTree<Record, [f64; 2]> = {
        let encoded = include_bytes!("data/output.bin");
        deserialize(&encoded[..]).unwrap()
    };
}

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
