use std::cmp;

pub fn p1(s: &String) -> String {
    let (earliest_time, v) = parse(s);
    let v: Vec<i32> = v[..].iter().map(|x| *x).filter(|x| *x > 0).collect();

    let (mut min_id, mut min_val) = (-1, earliest_time);
    for x in v {
        let val = x - earliest_time % x;

        if val < min_val {
            min_id = x;
            min_val = val;
        }
    }

    return (min_id*min_val).to_string();
}

pub fn p2(s: &String) -> String {
    // (x + 0) % 07 = 0
    // (x + 1) % 13 = 0
    // (x + 4) % 59 = 0
    // (x + 6) % 31 = 0
    // (x + 7) % 19 = 0
    return "".to_string();
}

fn get_partitions<'a>(v: &'a Vec<i32>) -> (Vec<(&'a i32, &'a i32)>, Vec<(&'a i32, &'a i32)>) {
    let pairs: Vec<(&i32, &i32)> = v[..v.len()-1].iter().zip(v[1..v.len()].iter()).collect();
    let (ones, threes): (Vec<(&i32, &i32)>, Vec<(&i32, &i32)>) = pairs.iter().partition(|&x| (*x).1 - (*x).0 == 1);
    return (ones, threes);
}

fn parse(s: &String) -> (i32, Vec<i32>) {
    let earliest_time: i32 = s.lines().nth(0).unwrap().parse::<i32>().unwrap();
    let v = s.lines().nth(1).unwrap().split(",").map(|x| match x.parse::<i32>() { Ok(v) => { v }, Err(_) => { 0 }}).collect();
    return (earliest_time, v);
}
