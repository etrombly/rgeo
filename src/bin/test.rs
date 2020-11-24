use rgeo::*;

fn main() {
    println!("{:?}", search(44.353_34_f32, -72.740_23_f32));
    println!("{:?}", nearest(44.353_34_f32, -72.740_23_f32, 6));
}
