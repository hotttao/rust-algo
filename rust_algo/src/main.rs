

mod a;
mod stock;


fn main() {
    println!("Hello, world!");
    a::greet();
    stock::d::goodbye();
    stock::d::c::hello();
    stock::b::b_goodbye();
}
