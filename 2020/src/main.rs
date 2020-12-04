use std::io;
use std::io::prelude::*;
use std::env;

mod day;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Usage: ./advent2020 [01/1 01/2] [02/1 02/2] ...");
        std::process::exit(1);
    }

    let mut buffer = String::new();
    let _ = io::stdin().read_to_string(&mut buffer);

    for w in &args[1..] {
        match w.as_str() {
            "01/1" => println!("prob 01/1: {}", day::_01::p1(&buffer)),
            "01/2" => println!("prob 01/2: {}", day::_01::p2(&buffer)),
            "02/1" => println!("prob 02/1: {}", day::_02::p1(&buffer)),
            "02/2" => println!("prob 02/2: {}", day::_02::p2(&buffer)),
            "03/1" => println!("prob 03/1: {}", day::_03::p1(&buffer)),
            "03/2" => println!("prob 03/2: {}", day::_03::p2(&buffer)),
            _ => ()
        }
    }
}
