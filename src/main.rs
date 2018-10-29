extern crate rand;

use std::env;

fn main() {
    let mut price: u32 = rand::random();
    let mut quality: u32 = rand::random();
    let mut coeff: f64 = 1.0;

    for arg in env::args().skip(1) {
        let mut iter = arg.split('=');
        let mut key = match iter.next() {
            Some(k) => k,
            None => {
                eprintln!("Error obtaining key from arg {:?}", arg);
                continue;
            }
        };
        let mut val = match iter.next() {
            Some(v) => v,
            None => {
                eprintln!("Error obtaining key from arg {:?}", arg);
                continue;
            }
        };

        match key {
            "price" | "p" => {
                price = val.parse().unwrap();
            }
            "quality" | "q" => {
                quality = val.parse().unwrap();
            }
            "coeff" | "c" => {
                coeff = val.parse().unwrap();
            }
            _other => {
                eprintln!("Unrecognized argument {:?}", arg);
            }
        }
    }

    let metric: f64 = (quality as f64).log2() + (price as f64).log2() * coeff;

    println!("price: {}", price);
    println!("quality: {}", quality);
    println!("coeff: {}", coeff);
    println!("metric: {}", metric);
}
