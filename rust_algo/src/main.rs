
mod a;
mod stock;

use crate::stock::b;
use rust_algo::foo;

fn main() {
    println!("Hello, world!");
    a::greet();
    stock::d::goodbye();
    stock::d::c::hello();
    b::b_goodbye();
    foo::hello();
}
