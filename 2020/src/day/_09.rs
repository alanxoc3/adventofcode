pub fn p1(s: &String) -> String {
    let v = parse(s);
    return get_invalid_number(&v).to_string();
}

pub fn p2(s: &String) -> String {
    let v = parse(s);
    let invalid = get_invalid_number(&v);

    for i in 0..v.len() {
        let mut count = 0;
        for j in i..v.len() {
            count += v[j];
            if count == invalid {
                let min = v[i..j].iter().cloned().fold(v[j], i64::min);
                let max = v[i..j].iter().cloned().fold(v[j], i64::max);
                return (min + max).to_string();
            } else if count > invalid {
                break;
            }
        }
    }

    return "Problem with puzzle input.".to_string();
}

fn get_invalid_number(v: &Vec<i64>) -> i64 {
    for i in 25..v.len() {
        let preamble_combos = comb(&v[i-25..i], 2);
        let result = preamble_combos.iter().filter(|x| x[0] + x[1] == v[i]).next();
        if result.is_none() {
            return v[i];
        }
    }
    return -1;
}

fn parse(s: &String) -> Vec<i64> {
    return s.lines().map(|l| l.parse::<i64>().unwrap()).collect();
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
