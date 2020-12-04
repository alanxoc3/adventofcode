pub fn p1(s: &String) -> String {
    let list = parse_to_string_vec(s);
    return count_trees(&list, 3, 1).to_string();
}

pub fn p2(s: &String) -> String {
    let list = parse_to_string_vec(s);
    let mut valid_count: u64 = 1;

    valid_count *= count_trees(&list, 1, 1) as u64;
    valid_count *= count_trees(&list, 3, 1) as u64;
    valid_count *= count_trees(&list, 5, 1) as u64;
    valid_count *= count_trees(&list, 7, 1) as u64;
    valid_count *= count_trees(&list, 1, 2) as u64;

    return valid_count.to_string();
}

fn parse_to_string_vec(s: &String) -> Vec<String> {
    let mut vec: Vec<String> = vec![];

    for l in s.lines() {
        if l.len() > 0 {
            vec.push(l.to_string());
        }
    }

    return vec
}

fn count_trees(list: &Vec<String>, dx: usize, dy: usize) -> i32 {
    let mut valid_count = 0;
    let mut xpos = 0;

    for line in list.iter().step_by(dy) {
        let len = line.chars().count();

        let curr_char = line.chars().nth(xpos % len).unwrap();
        if curr_char == '#' {
            valid_count += 1;
        }

        xpos += dx
    }

    return valid_count;
}
