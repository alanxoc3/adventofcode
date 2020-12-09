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
            "04/1" => println!("prob 04/1: {}", day::_04::p1(&buffer)),
            "04/2" => println!("prob 04/2: {}", day::_04::p2(&buffer)),
            "05/1" => println!("prob 05/1: {}", day::_05::p1(&buffer)),
            "05/2" => println!("prob 05/2: {}", day::_05::p2(&buffer)),
            "06/1" => println!("prob 06/1: {}", day::_06::p1(&buffer)),
            "06/2" => println!("prob 06/2: {}", day::_06::p2(&buffer)),
            // "07/1" => println!("prob 07/1: {}", day::_07::p1(&buffer)),
            // "07/2" => println!("prob 07/2: {}", day::_07::p2(&buffer)),
            // "08/1" => println!("prob 08/1: {}", day::_08::p1(&buffer)),
            // "08/2" => println!("prob 08/2: {}", day::_08::p2(&buffer)),
            // "09/1" => println!("prob 09/1: {}", day::_09::p1(&buffer)),
            // "09/2" => println!("prob 09/2: {}", day::_09::p2(&buffer)),
            // "10/1" => println!("prob 10/1: {}", day::_10::p1(&buffer)),
            // "10/2" => println!("prob 10/2: {}", day::_10::p2(&buffer)),
            // "11/1" => println!("prob 11/1: {}", day::_11::p1(&buffer)),
            // "11/2" => println!("prob 11/2: {}", day::_11::p2(&buffer)),
            // "12/1" => println!("prob 12/1: {}", day::_12::p1(&buffer)),
            // "12/2" => println!("prob 12/2: {}", day::_12::p2(&buffer)),
            // "13/1" => println!("prob 13/1: {}", day::_13::p1(&buffer)),
            // "13/2" => println!("prob 13/2: {}", day::_13::p2(&buffer)),
            // "14/1" => println!("prob 14/1: {}", day::_14::p1(&buffer)),
            // "14/2" => println!("prob 14/2: {}", day::_14::p2(&buffer)),
            // "15/1" => println!("prob 15/1: {}", day::_15::p1(&buffer)),
            // "15/2" => println!("prob 15/2: {}", day::_15::p2(&buffer)),
            // "16/1" => println!("prob 16/1: {}", day::_16::p1(&buffer)),
            // "16/2" => println!("prob 16/2: {}", day::_16::p2(&buffer)),
            // "17/1" => println!("prob 17/1: {}", day::_17::p1(&buffer)),
            // "17/2" => println!("prob 17/2: {}", day::_17::p2(&buffer)),
            // "18/1" => println!("prob 18/1: {}", day::_18::p1(&buffer)),
            // "18/2" => println!("prob 18/2: {}", day::_18::p2(&buffer)),
            // "19/1" => println!("prob 19/1: {}", day::_19::p1(&buffer)),
            // "19/2" => println!("prob 19/2: {}", day::_19::p2(&buffer)),
            // "20/1" => println!("prob 20/1: {}", day::_20::p1(&buffer)),
            // "20/2" => println!("prob 20/2: {}", day::_20::p2(&buffer)),
            // "21/1" => println!("prob 21/1: {}", day::_21::p1(&buffer)),
            // "21/2" => println!("prob 21/2: {}", day::_21::p2(&buffer)),
            // "22/1" => println!("prob 22/1: {}", day::_22::p1(&buffer)),
            // "22/2" => println!("prob 22/2: {}", day::_22::p2(&buffer)),
            // "23/1" => println!("prob 23/1: {}", day::_23::p1(&buffer)),
            // "23/2" => println!("prob 23/2: {}", day::_23::p2(&buffer)),
            // "24/1" => println!("prob 24/1: {}", day::_24::p1(&buffer)),
            // "24/2" => println!("prob 24/2: {}", day::_24::p2(&buffer)),
            // "25/1" => println!("prob 25/1: {}", day::_25::p1(&buffer)),
            // "25/2" => println!("prob 25/2: {}", day::_25::p2(&buffer)),
            _ => ()
        }
    }
}
