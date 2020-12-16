pub fn p1(s: &String) -> String {
    let (earliest_time, v) = parse(s);
    let v: Vec<i64> = v[..].iter().map(|x| *x).filter(|x| *x > 0).collect();

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
    let (_, v) = parse(s);
    let v: Vec<(i64, i64)> = v.iter()
        .enumerate()
        .filter(|p| *p.1 > 0)
        .map(|p| (*p.1, (p.1 - p.0 as i64) % p.1))
        .map(|p| (p.0, (p.1 + p.0) % p.0))
        .collect();

    let modulii: Vec<_> = v.iter().map(|p| p.0).collect();
    let residues: Vec<_> = v.iter().map(|p| p.1).collect();
    return chinese_remainder(&residues, &modulii).unwrap().to_string();
}

// Chinese Remainder Theorem Implementation.
// Stolen from: https://rosettacode.org/wiki/Chinese_remainder_theorem
fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}

fn parse(s: &String) -> (i64, Vec<i64>) {
    let earliest_time: i64 = s.lines().nth(0).unwrap().parse::<i64>().unwrap();
    let v = s.lines().nth(1).unwrap().split(",").map(|x| match x.parse::<i64>() { Ok(v) => { v }, Err(_) => { 0 }}).collect();
    return (earliest_time, v);
}
