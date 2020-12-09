use std::collections::HashMap;

pub fn p1(s: &String) -> String {
    let lm = parse(s);
    let mut count = 0;

    for m in lm {
        count += m.len() - 1;
    }

    return count.to_string();
}

pub fn p2(s: &String) -> String {
    let lm = parse(s);
    let mut count = 0;

    for m in lm {
        let num_of_items = m.get(&'!').unwrap();
        for (key, value) in m.iter() {
            if *key != '!' && value == num_of_items {
                count += 1;
            }
        }
    }

    return count.to_string();
}

fn parse(s: &String) -> Vec<HashMap<char, i32>> {
    let mut list = Vec::new();
    let mut m = HashMap::new();
    m.insert('!', 0);

    for l in s.lines() {
        if l.len() == 0 {
            if m.len() > 0 {
                list.push(m);
            }
            m = HashMap::new();
            m.insert('!', 0);
        } else {
            if m.contains_key(&'!') {
                m.insert('!', *m.get(&'!').unwrap() + 1);
            }
        }

        for c in l.chars() {
            if m.contains_key(&c) {
                m.insert(c, *m.get(&c).unwrap() + 1);
            } else {
                m.insert(c, 1);
            }
        }
    }

    if m.len() > 0 {
        list.push(m);
    }

    return list
}
