use std::cmp;

pub fn p1(s: &String) -> String {
    let mut max_id = -1;
    let seats = parse_seats(s);

    for s in seats {
        max_id = cmp::max(s.0 * 8 + s.1, max_id)
    }

    return max_id.to_string();
}

pub fn p2(s: &String) -> String {
    let seats = parse_seats(s);
    let mut sids = Vec::new();

    for s in seats {
        sids.push(s.0 * 8 + s.1);
    }

    sids.sort();
    let mut prev_sid = -1;
    for s in sids {
        if s - prev_sid > 1 && prev_sid != -1 {
            prev_sid = s - 1;
            break;
        }
        prev_sid = s;
    }

    return prev_sid.to_string();
}

fn calc_binary_divide(input: &str, f: char, b: char) -> i32 {
    let mut max_num = (2 as i32).pow(input.len() as u32) - 1;
    let mut min_num = 0;

    for x in input.chars() {
        let step = (max_num-min_num) / 2 + 1;

        if x == f {
            max_num -= step
        } else if x == b {
            min_num += step
        } else {
            assert!(false)
        }
    }

    assert!(max_num == min_num);
    return max_num
}

fn parse_seats(s: &String) -> Vec<(i32, i32)> {
    let mut seats = Vec::new();

    for l in s.lines() {
        if l.len() != 10 { continue }
        let row = calc_binary_divide(&l[..7], 'F', 'B');
        let col = calc_binary_divide(&l[7..], 'L', 'R');
        seats.push((row, col));
    }

    return seats
}
