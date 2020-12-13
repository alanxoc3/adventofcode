use regex::Regex;
use std::collections::HashSet;

pub fn p1(s: &String) -> String {
    let v = parse(s);
    let (_, _, count) = run_prog(&v);
    return count.to_string();
}

pub fn p2(s: &String) -> String {
    let mut v = parse(s);

    for i in 0..v.len() {
        if v[i].command != "acc" {
            let old_cmd = &v[i].command;
            let new_cmd = if old_cmd == "jmp" { "nop" } else { "jmp" };
            let replacement = Instruction { command: new_cmd.to_string(), num: v[i].num };

            let new_vec = [&v[..i], &vec![replacement], &v[i+1..]].concat();
            let (_, ind, count) = run_prog(&new_vec);
            if ind == new_vec.len() as i32 {
                return count.to_string();
            }
            v[i].command = (&old_cmd).to_string();
        }
    }

    return "Problem with puzzle input.".to_string();
}

fn run_prog(v: &Vec<Instruction>) -> (HashSet<i32>, i32, i32) {
    let mut set: HashSet<i32> = HashSet::new();
    let mut count = 0;
    let mut ind = 0;

    while !set.contains(&ind) {
        if ind as usize >= v.len() { break; }
        set.insert(ind);

        let ins = v.get(ind as usize).unwrap();
        if ins.command == "acc" {
            count += ins.num;
            ind += 1;
        } else if ins.command == "jmp" {
            ind += ins.num;
        } else if ins.command == "nop" {
            ind += 1;
        }
    }

    return (set, ind, count);
}

#[derive(Clone)]
#[derive(Debug)]
struct Instruction {
    command: String,
    num: i32,
}

fn parse(s: &String) -> Vec<Instruction> {
    let re = Regex::new(r"^(\w+) (\+|-)(\d+)$").unwrap();
    let mut v = Vec::new();

    for l in s.lines() {
        let cap = re.captures(l).unwrap();
        let num = &cap[3].to_string().parse::<i32>().unwrap();
        let num = if &cap[2] == "+" { *num } else { -*num };
        let ins = Instruction { command: cap[1].to_string(), num: num };
        v.push(ins);
    }

    return v;
}
