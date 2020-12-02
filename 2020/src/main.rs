use std::io;
use std::io::prelude::*;
use std::env;

mod days;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Usage: ./advent2020 [01p1 01p2...]");
        std::process::exit(1);
    }

    let mut buffer = String::new();
    let _ = io::stdin().read_to_string(&mut buffer);

    for w in &args[1..] {
        match w.as_str() {
            "01p1" => println!("{}", days::day01p1(&buffer)),
            "01p2" => println!("{}", days::day01p2(&buffer)),
            _ => ()
        }
    }
}
