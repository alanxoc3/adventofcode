pub fn p1(s: &String) -> String {
    let list = parse_to_pass_info(s);
    let mut valid_count = 0;

    for pi in list {
        let char_count = pi.password.matches(pi.character).count() as u32;
        if pi.min_count <= char_count && char_count <= pi.max_count {
            valid_count += 1;
        }
    }

    return valid_count.to_string();
}

pub fn p2(s: &String) -> String {
    let list = parse_to_pass_info(s);
    let mut valid_count = 0;

    for pi in list {
        let mut is_char_here = false;
        match pi.password.chars().nth(pi.min_count as usize - 1) {
            Some(c) => {
                if c == pi.character {
                    is_char_here = true;
                }
            }, None => {}
        }

        match pi.password.chars().nth(pi.max_count as usize - 1) {
            Some(c) => {
                is_char_here = is_char_here ^ (c == pi.character)
            }, None => {}
        }

        if is_char_here {
            valid_count += 1;
        }
    }

    return valid_count.to_string();
}

#[derive(Debug)]
#[derive(Default)]
struct PassInfo {
    min_count: u32,
    max_count: u32,
    character: char,
    password: String,
}

fn parse_to_pass_info(s: &String) -> Vec<PassInfo> {
    let mut vec: Vec<PassInfo> = vec![];

    for l in s.lines() {
        let words: Vec<&str> = l.split(" ").collect();
        let mut pi: PassInfo = Default::default();

        match words.get(0) {
            Some(w) => {
                let nums: Vec<&str> = w.split("-").collect();
                match nums.get(0) {
                    Some(nt) => {
                        match nt.parse::<u32>() {
                            Ok(n) => {
                                pi.min_count = n
                            }, Err(_) => { break }
                        }
                    }, None => { break }
                }

                match nums.get(1) {
                    Some(nt) => {
                        match nt.parse::<u32>() {
                            Ok(n) => {
                                pi.max_count = n
                            }, Err(_) => { break }
                        }
                    }, None => { break }
                }
            }, None => { break }
        }

        match words.get(1) {
            Some(w) => {
                pi.character = w.chars().next().unwrap()
            }, None => { break }
        }

        match words.get(2) {
            Some(w) => {
                pi.password = w.to_string()
            }, None => { break }
        }

        vec.push(pi)
    }

    return vec
}
