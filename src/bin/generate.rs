#[macro_use]
extern crate serde_derive;

use bincode::serialize;
use csv::ReaderBuilder;
use kdtree::KdTree;
use rgeo::record::{Record, Nvec};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
struct CountryCode {
    name: String,
    #[serde(rename = "alpha-2")]
    code: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct FullRecord {
    geonameid: u32,
    name: String,
    latitude: f32,
    longitude: f32,
    #[serde(rename = "feature code")]
    feature_code: String,
    #[serde(rename = "country code")]
    country: String,
    population: i64,
}

impl FullRecord {
    fn into_record(self) -> Record {
        Record {
            name: self.name,
            nvec: Nvec::from_lat_long(self.latitude, self.longitude),
            country: self.country,
        }
    }

    //fn to_country(self) -> Country {
    //    Country { geonameid: self.geonameid, name: self.name, country: self.country }
    //}
}

fn main() {
    let mut country_file = File::open("data/countries_iso.csv").unwrap();
    let mut csv = "".to_string();
    country_file.read_to_string(&mut csv).unwrap();
    let mut rdr = ReaderBuilder::new().from_reader(csv.as_bytes());
    let mut codes = HashMap::new();
    for result in rdr.deserialize() {
        let record: CountryCode = result.unwrap();
        codes.insert(record.code, record.name);
    }

    let mut csv = "geonameid\tname\tasciiname\talternatenames\tlatitude\tlongitude\tfeature class\tfeature code\tcountry code\tcc2\tadmin1 code\tadmin2 code\tadmin3 code\tadmin4 code\tpopulation\televation\tdem\ttimezone\tmodification date\n".to_string();
    let mut in_file = File::open("data/allCountries.txt").unwrap();
    in_file.read_to_string(&mut csv).unwrap();

    let mut tree = KdTree::new(3);

    let mut rdr = ReaderBuilder::new()
        .delimiter(b'\t')
        .from_reader(csv.as_bytes());

    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: FullRecord = result.unwrap();
        //if record.feature_code == "PCLI".to_string() {
        //    countries.insert(record.geonameid, record.to_country());
        //}
        if record.population > 100 {
            let mut record = record.into_record();
            if codes.contains_key(&record.country) {
                record.country = codes[&record.country].clone();
            }
            tree.add([record.nvec.x, record.nvec.y, record.nvec.z], record)
                .unwrap();
        }
    }

    let encoded: Vec<u8> = serialize(&tree).unwrap();
    let mut bin = File::create("data/output.bin").unwrap();
    bin.write_all(&encoded).unwrap();
    bin.sync_all().unwrap();
}
