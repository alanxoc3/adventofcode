use std::collections::HashMap;
use std::cmp;

pub fn p1(s: &String) -> String {
    let (mut m, _, _) = parse(s);
    let mut dm = HashMap::new();
    let mut did_change = true;
    let mut occupied_seats = 0;

    while did_change {
        occupied_seats = 0;
        did_change = false;

        for (coord, val) in &m {
            let new_val = seat_change(&m, &coord, 1, 1, 4);
            dm.insert(*coord, new_val);
            if new_val != *val { did_change = true; }
            occupied_seats += new_val as i32;
        }

        for (coord, val) in &dm {
            m.insert(*coord, *val);
        }
    }

    return occupied_seats.to_string();
}

pub fn p2(s: &String) -> String {
    let (mut m, max_row, max_col) = parse(s);
    let mut dm = HashMap::new();
    let mut did_change = true;
    let mut occupied_seats = 0;

    while did_change {
        occupied_seats = 0;
        did_change = false;

        for (coord, val) in &m {
            let new_val = seat_change(&m, &coord, max_row as i32, max_col as i32, 5);
            dm.insert(*coord, new_val);
            if new_val != *val { did_change = true; }
            occupied_seats += new_val as i32;
        }

        for (coord, val) in &dm {
            m.insert(*coord, *val);
        }
    }

    return occupied_seats.to_string();
}

fn seat_change(m: &HashMap<(i32, i32), bool>, pos: &(i32, i32), reach_row: i32, reach_col: i32, tolerance: i32) -> bool {
    let seat_val = *m.get(&(pos.0,pos.1)).unwrap_or(&false);
    let (mut n, mut nw, mut w, mut sw, mut s, mut se, mut e, mut ne) = (0, 0, 0, 0, 0, 0, 0, 0);
    let reach_diag = cmp::min(reach_row, reach_col);

    for i in 1..reach_row+1 { match m.get(&(pos.0-i, pos.1)) { Some(v) => { n = *v as i32; break }, None => {} } }
    for i in 1..reach_row+1 { match m.get(&(pos.0+i, pos.1)) { Some(v) => { s = *v as i32; break }, None => {} } }
    for i in 1..reach_col+1 { match m.get(&(pos.0, pos.1-i)) { Some(v) => { w = *v as i32; break }, None => {} } }
    for i in 1..reach_col+1 { match m.get(&(pos.0, pos.1+i)) { Some(v) => { e = *v as i32; break }, None => {} } }

    for i in 1..reach_diag+1 { match m.get(&(pos.0-i, pos.1-i)) { Some(v) => { nw = *v as i32; break }, None => {} } }
    for i in 1..reach_diag+1 { match m.get(&(pos.0+i, pos.1+i)) { Some(v) => { se = *v as i32; break }, None => {} } }
    for i in 1..reach_diag+1 { match m.get(&(pos.0+i, pos.1-i)) { Some(v) => { sw = *v as i32; break }, None => {} } }
    for i in 1..reach_diag+1 { match m.get(&(pos.0-i, pos.1+i)) { Some(v) => { ne = *v as i32; break }, None => {} } }

    let occupied_seats = n + nw + w + sw + s + se + e + ne;
    if seat_val && occupied_seats >= tolerance {
        return false;
    } else if !seat_val && occupied_seats == 0 {
        return true;
    } else {
        return seat_val;
    }
}

fn parse(s: &String) -> (HashMap<(i32,i32), bool>, usize, usize) {
    let mut m = HashMap::new();
    let (mut max_row, mut max_col) = (0, 0);

    for (row, l) in s.lines().enumerate() {
        for (col, c) in l.chars().enumerate() {
            if c == 'L' {
                if col > max_col { max_col = col; }
                if row > max_row { max_row = row; }
                m.insert((row as i32, col as i32), false);
            }
        }
    }

    return (m, max_row, max_col);
}
