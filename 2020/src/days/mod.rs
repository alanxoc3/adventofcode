pub fn day01p1(s: &String) -> String {
    let list = parse_to_num_list(s);
    let combos = comb(&list[..], 2);

    for pair in combos {
        if pair[0] + pair[1] == 2020 {
            return (pair[0] * pair[1]).to_string();
        }
    }

    return "".to_string();
}

pub fn day01p2(s: &String) -> String {
    let list = parse_to_num_list(s);
    let combos = comb(&list[..], 3);

    for pair in combos {
        if pair[0] + pair[1] + pair[2] == 2020 {
            return (pair[0] * pair[1] * pair[2]).to_string();
        }
    }

    return "".to_string();
}

fn parse_to_num_list(s: &String) -> Vec<i32> {
    let mut vec: Vec<i32> = vec![];

    for (pos, l) in s.lines().enumerate() {
        match l.chars().next() {
            Some(_) => {
                match &l[0..].parse::<i32>() {
                    Ok(n) => {
                        vec.push(*n)
                    }, Err(_) => {
                        println!("Problem with text '{}' on line #{}.", &l[0..], pos+1)
                    }

                }
            },
            None => {}
        }
    }

    return vec
}

// Stolen from: https://rosettacode.org/wiki/Combinations
fn comb<T>(slice: &[T], k: usize) -> Vec<Vec<T>>
where
T: Copy,
{
    // If k == 1, return a vector containing a vector for each element of the slice.
    if k == 1 {
        return slice.iter().map(|x| vec![*x]).collect::<Vec<Vec<T>>>();
    }
    // If k is exactly the slice length, return the slice inside a vector.
    if k == slice.len() {
        return vec![slice.to_vec()];
    }

    // Make a vector from the first element + all combinations of k - 1 elements of the rest of the slice.
    let mut result = comb(&slice[1..], k - 1)
        .into_iter()
        .map(|x| [&slice[..1], x.as_slice()].concat())
        .collect::<Vec<Vec<T>>>();

    // Extend this last vector with the all the combinations of k elements after from index 1 onward.
    result.extend(comb(&slice[1..], k));
    // Return final vector.
    return result;
}
