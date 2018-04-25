extern crate csv;
extern crate bincode;
extern crate kdtree;
extern crate rgeo;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use csv::ReaderBuilder;
use bincode::serialize;
use kdtree::KdTree;
use rgeo::record::Record;
use rgeo::country::Country;

#[derive(Debug, Deserialize, Serialize)]
struct FullRecord {
    geonameid: u32,
    name: String,
    latitude: f64,
    longitude: f64,
    #[serde(rename="feature code")]
    feature_code: String,
    #[serde(rename="country code")]
    country: String,
    population: i64,
}

#[derive(Serialize, Deserialize)]
struct Shapes {
    #[serde(rename="type")]
    type_name: String,
    features: Vec<Feature>,
}

#[derive(Serialize, Deserialize)]
struct Feature {
    #[serde(rename="type")]
    type_name: String,
    properties: Property,
    geometry: Geometry,
}

#[derive(Serialize, Deserialize)]
struct Property {
    #[serde(rename="geoNameId", deserialize_with = "parse_geonameid")]
    geonameid: u32,
}

type Polygon = Vec<Vec<[f64;2]>>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "coordinates")]
enum Geometry {
    Polygon(Polygon),
    MultiPolygon(Vec<Polygon>),
}

fn parse_geonameid<'de, D>(de: D) -> Result<u32, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let deser_result: serde_json::Value = try!(serde::Deserialize::deserialize(de));
    match deser_result {
        serde_json::Value::String(ref s) => Ok(s.parse::<u32>().unwrap()),
        _ => Err(serde::de::Error::custom("Unexpected value")),
    }
}

impl FullRecord {
    fn to_record(self) -> Record {
        Record{ name: self.name, latitude: self.latitude, longitude: self.longitude, country: self.country }
    }

    fn to_country(self) -> Country {
        Country { geonameid: self.geonameid, name: self.name, country: self.country }
    }
}

fn main() {
    let mut csv: String = "geonameid\tname\tasciiname\talternatenames\tlatitude\tlongitude\tfeature class\tfeature code\tcountry code\tcc2\tadmin1 code\tadmin2 code\tadmin3 code\tadmin4 code\tpopulation\televation\tdem\ttimezone\tmodification date\n".to_string();
    let mut in_file = File::open("data/allCountries.txt").unwrap();
    in_file.read_to_string(&mut csv).unwrap();

    let mut tree = KdTree::new(2);

    let mut countries: HashMap<u32, Country> = HashMap::new();

    let mut rdr = ReaderBuilder::new()
         .delimiter(b'\t')
         .from_reader(csv.as_bytes());

    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: FullRecord = result.unwrap();
        if record.feature_code == "PCLI".to_string() {
            countries.insert(record.geonameid, record.to_country());
        } else if record.population > 100 {
            tree.add([record.latitude, record.longitude], record.to_record()).unwrap();
        }
    }

    println!("{:?}", countries);
    let encoded: Vec<u8> = serialize(&tree).unwrap();
    let mut bin = File::create("data/output.bin").unwrap();
    bin.write_all(&encoded).unwrap();
    bin.sync_all().unwrap();
}