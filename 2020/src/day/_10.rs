use std::cmp;

pub fn p1(s: &String) -> String {
    let v = parse(s);
    let (ones, threes) = get_partitions(&v);
    return (ones.len() * (threes.len()+1)).to_string();
}

pub fn p2(s: &String) -> String {
    // 2 ^ (n - 1) - (2 ^ max(0, n-3) - 1)
    let v = parse(s);
    let (_, threes) = get_partitions(&v);

    let mut last_end = 0;
    let mut gaps = vec![];
    for pair in &threes {
        let diff = pair.0 - last_end;
        if diff > 0 {
            gaps.push(diff);
        }
        last_end = *pair.1;
    }

    let mut acc: i64 = 1;
    for n in gaps {
        let pow = 2i64.pow(n as u32 - 1) - (2i64.pow(cmp::max(0, n-3) as u32)-1);
        acc *= pow;
    }

    return acc.to_string();
}

fn get_partitions<'a>(v: &'a Vec<i32>) -> (Vec<(&'a i32, &'a i32)>, Vec<(&'a i32, &'a i32)>) {
    let pairs: Vec<(&i32, &i32)> = v[..v.len()-1].iter().zip(v[1..v.len()].iter()).collect();
    let (ones, threes): (Vec<(&i32, &i32)>, Vec<(&i32, &i32)>) = pairs.iter().partition(|&x| (*x).1 - (*x).0 == 1);
    return (ones, threes);
}

fn parse(s: &String) -> Vec<i32> {
    let mut v: Vec<i32> = s.lines().map(|l| l.parse::<i32>().unwrap()).collect();
    v.push(0);
    v.sort();
    return v;
}
