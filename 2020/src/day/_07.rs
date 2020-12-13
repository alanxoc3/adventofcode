use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn p1(s: &String) -> String {
    let m = parse(s);
    let mut v = vec!["shiny gold"];
    let mut i = 0;

    while i < v.len() {
       for (k, bags) in &m {
           for b in bags {
               if b.color == v[i] {
                   v.push(&k);
               }
           }
       }
       i += 1;
    }

    let set: HashSet<_> = v.iter().cloned().collect();
    return (set.len()-1).to_string();
}

pub fn p2(s: &String) -> String {
    let m = parse(s);
    let num = count_bags(&m, "shiny gold");
    return (num - 1).to_string();
}

fn count_bags(m: &HashMap<String, Vec<Bag>>, c: &str) -> i32 {
    let v = &m[c];
    let mut count = 1;

    for b in v {
        count += b.count * count_bags(&m, &b.color);
    }

    return count;
}

#[derive(Debug)]
struct Bag {
    color: String,
    count: i32,
}

fn parse(s: &String) -> HashMap<String, Vec<Bag>> {
    let line_re = Regex::new(r"^(\w+ \w+) bags? contain (no other bags|(\d+ \w+ \w+ bags?(, )?)+)\.$").unwrap();
    let bags_re = Regex::new(r"(\d+) (\w+ \w+)").unwrap();
    let mut m = HashMap::new();

    for l in s.lines() {
        let cap = line_re.captures(l).unwrap();
        let bags = bags_re.captures_iter(&cap[2]).map(|x| Bag { color: x[2].to_string(), count: x[1].to_string().parse::<i32>().unwrap() }).collect();
        m.insert(cap[1].to_string(), bags);
    }

    return m;
}
