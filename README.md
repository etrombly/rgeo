# rgeo

Rust library for reverse geocoding. Uses data from the geonames database. http://www.geonames.org/ (http://download.geonames.org/export/dump/allCountries.zip)

Usage:
```
extern crate rgeo;
use rgeo::{search, nearest};

search(44.353339_f32, -72.740231_f32);
// Some((0.0001186200820000013, Record { name: "Village of Waterbury", latitude: 44.34279, longitude: -72.74294, country: "US" }))

nearest(44.353_339_f32, -72.740_231_f32, 6);
// Some([(0.00011866877, Record { name: "Village of Waterbury", latitude: 44.34279, longitude: -72.74294, country: "US" }), (0.00049654034, Record { name: "Waterbury", latitude: 44.33783, longitude: -72.75623, country: "US" }), (0.0010441553, Record { name: "Town of Waterbury", latitude: 44.38513, longitude: -72.74602, country: "US" }), (0.009455653, Record { name: "Town of Duxbury", latitude: 44.30492, longitude: -72.82456, country: "US" }), (0.0100118, Record { name: "Town of Moretown", latitude: 44.25658, longitude: -72.71475, country: "US" }), (0.010925336, Record { name: "Moretown", latitude: 44.25089, longitude: -72.76095, country: "US" })])
```

The offline data takes about 19M with the default settings. Locations with a population of less than 100 aren't included.

Benchmarks:

with kdtree:
```
search                  time:   [1.2217 us 1.2234 us 1.2254 us]                    
    Found 16 outliers among 100 measurements (16.00%)
    3 (3.00%) high mild
    13 (13.00%) high severe
```

with kiddo:
```
search                  time:   [726.85 ns 730.24 ns 736.53 ns]                    
                        change: [-40.476% -40.291% -40.076%] (p = 0.00 < 0.05)
                        Performance has improved.
```

with kiddo and nearest_one:
```
search                  time:   [490.32 ns 492.09 ns 494.60 ns]                     
                        change: [-32.854% -32.427% -31.970%] (p = 0.00 < 0.05)
                        Performance has improved.
```