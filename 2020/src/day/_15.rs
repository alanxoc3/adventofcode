use std::collections::HashMap;

pub fn p1(s: &String) -> String {
    let v = parse(s);
    return game_at_pos(v, 2020).to_string();
}

pub fn p2(s: &String) -> String {
    let v = parse(s);
    return game_at_pos(v, 30000000).to_string();
}

fn game_at_pos(v: Vec<u64>, pos: usize) -> u64 {
    let mut m = HashMap::new();
    let mut prev_num = 0;

    for (i, x) in v.iter().enumerate() {
        m.insert(*x, (i+1) as u64);
        prev_num = *x;
    }

    for turn in v.len()+1..pos+1 {
        let next_num = match m.get(&prev_num) {
            Some(t) => {
                (turn as u64 - 1) - t
            }, None => {
                0
            }
        };

        m.insert(prev_num, turn as u64-1);
        prev_num = next_num;
    }

    return prev_num;
}

fn parse(s: &String) -> Vec<u64> {
    return s.lines().next().unwrap().split(",").map(|l| l.parse::<u64>().unwrap()).collect();
}
