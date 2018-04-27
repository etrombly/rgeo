extern crate rgeo;

use rgeo::*;

fn main() {
    println!("{:?}", search(44.353_339_f32, -72.740_231_f32));
    println!("{:?}", nearest(44.353_339_f32, -72.740_231_f32, 6));
}
