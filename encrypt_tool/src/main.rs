use std::env;
use std::fs;

fn main() {
    let args : Vec<String> = env::args().collect();

    let query = &args[1];

    let password = &args[2];

    let algo = &args[3];


    println!("Hello, world!");
}
