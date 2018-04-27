# rgeo

Rust library for reverse geocoding. Uses data from the geonames database. http://www.geonames.org/

Usage:
```
extern crate rgeo;
use rgeo::search;

search(44.353339_f64, -72.740231_f64);
// Some((0.0001186200820000013, Record { name: "Village of Waterbury", latitude: 44.34279, longitude: -72.74294, country: "US" }))
```

The offline data takes about 28M with the default settings. Locations with a population of less than 100 aren't included.