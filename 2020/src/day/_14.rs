use regex::Regex;
use std::collections::HashMap;

pub fn p1(s: &String) -> String {
    let v = parse(s);
    let mut m = HashMap::new();
    for x in v {
        let val = (x.value | x.ones_mask) & (!x.zeros_mask);
        m.insert(x.location, val);
    }

    return m.values().fold(0, |acc, x| acc + x).to_string();
}

pub fn p2(s: &String) -> String {
    let v = parse(s);
    let mut m = HashMap::new();
    for x in v {
        let val = x.location | x.ones_mask;
        let x_combo_len = 1 << x.x_mask.len();
        for i in 0..x_combo_len {
            let combo_val = x.x_mask.iter()
                .enumerate()
                .map(|x| (x.1, (i & (1 << x.0)) as u64 >> x.0))
                .fold(val, |acc, x| if x.1 == 0 { acc & (!(1u64 << x.0)) } else { acc | (x.1 << x.0)});
            m.insert(combo_val, x.value);
        }
    }

    return m.values().fold(0, |acc, x| acc + x).to_string();
}

#[derive(Debug)]
struct MemSetCmd {
    ones_mask: u64,
    zeros_mask: u64,
    x_mask: Vec<u64>,
    location: u64,
    value: u64,
}

fn bool_str_to_u64(s: &str, c: char) -> u64 {
    s.chars()
        .map(|x| if x == c { true as u64 } else { false as u64 })
        .rev()
        .enumerate()
        .fold(0u64, |acc, x| acc | (x.1 << x.0))
}

fn parse(s: &String) -> Vec<MemSetCmd> {
    let mut v = vec![];
    let mask_re = Regex::new(r"^mask = ([01X]{36})$").unwrap();
    let mem_re  = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
    let mut ones_mask = 0u64;
    let mut zeros_mask = 0u64;
    let mut x_mask_list = vec![];

    for l in s.lines() {
        if mask_re.is_match(l) {
            let cap = mask_re.captures(l).unwrap();
            ones_mask = bool_str_to_u64(&cap[1], '1');
            zeros_mask = bool_str_to_u64(&cap[1], '0');
            x_mask_list = l.chars()
                .rev()
                .enumerate()
                .filter(|x| x.1 == 'X')
                .map(|x| x.0 as u64)
                .collect();

        } else if mem_re.is_match(l) {
            let cap = mem_re.captures(l).unwrap();
            let location = cap[1].parse::<u64>().unwrap();
            let value = cap[2].parse::<u64>().unwrap();
            let x_mask = x_mask_list.clone();

            v.push(MemSetCmd { ones_mask, zeros_mask, x_mask, location, value });
        } else {
            println!("Problem with puzzle input.");
        }
    }

    return v;
}
