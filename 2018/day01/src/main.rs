use std::io;
use std::io::prelude::*;

fn main() {
    let mut buffer = String::new();

    io::stdin().read_to_string(&mut buffer);

    let mut num = 0;
    for l in buffer.lines() {
        match l.chars().next() {
            Some(c) => {
                if c == '+' {
                    num += &l[1..].parse::<i32>().unwrap()
                } else if c == '-' {
                    num -= &l[1..].parse::<i32>().unwrap()
                }
            },
            None => {

            }
        }
    }
    println!("{}", num);
}
