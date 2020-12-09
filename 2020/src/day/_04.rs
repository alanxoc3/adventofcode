use std::collections::HashMap;
use regex::Regex;

pub fn p1(s: &String) -> String {
    let mut valid_count = 0;
    let passports = parse_passports(s);

    for p in passports {
        if has_needed_fields(&p) {
            valid_count += 1;
        }
    }

    return valid_count.to_string();
}

pub fn p2(s: &String) -> String {
    let mut valid_count = 0;
    let passports = parse_passports(s);

    for p in passports {
        if fields_valid(&p) {
            valid_count += 1;
        }
    }

    return valid_count.to_string();
}

fn has_needed_fields(p: &HashMap<&str, &str>) -> bool {
    return p.get("byr").is_some() && p.get("iyr").is_some() && p.get("eyr").is_some() && p.get("hgt").is_some() && p.get("hcl").is_some() && p.get("ecl").is_some() && p.get("pid").is_some();
}

fn fields_valid(p: &HashMap<&str, &str>) -> bool {
    if !has_needed_fields(p) {
        return false;
    }

    match p.get("byr").unwrap().parse::<i32>() {
        Ok(byr) => {
            if byr < 1920 || byr > 2002 { return false; }
        }, Err(_) => { return false; }
    }

    match p.get("iyr").unwrap().parse::<i32>() {
        Ok(byr) => {
            if byr < 2010 || byr > 2020 { return false; }
        }, Err(_) => { return false; }
    }

    match p.get("eyr").unwrap().parse::<i32>() {
        Ok(byr) => {
            if byr < 2020 || byr > 2030 { return false; }
        }, Err(_) => { return false; }
    }

    let mut hgt = p.get("hgt").unwrap().to_string();
    if hgt.ends_with("in") {
        hgt.pop(); hgt.pop();

        match hgt.parse::<i32>() {
            Ok(n) => {
                if n < 59 || n > 76 { return false; }
            }, Err(_) => { return false; }
        }
    } else if hgt.ends_with("cm") {
        hgt.pop(); hgt.pop();

        match hgt.parse::<i32>() {
            Ok(n) => {
                if n < 150 || n > 193 { return false; }
            }, Err(_) => { return false; }
        }
    } else {
        return false;
    }

    let hcl = p.get("hcl").unwrap();
    if !Regex::new(r"^#[0-9a-fA-F]{6}$").unwrap().is_match(hcl) { return false; }

    match *p.get("ecl").unwrap() {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => { },
        _ => { return false; }
    }

    let pid = p.get("pid").unwrap();
    if !Regex::new(r"^[0-9]{9}$").unwrap().is_match(pid) { return false; }

    return true;
}

fn parse_passports(s: &String) -> Vec<HashMap<&str, &str>> {
    let mut passports = Vec::new();
    let mut current_passport: HashMap<&str, &str> = HashMap::new();

    for l in s.lines() {
        if l.len() > 0 {
            let words: Vec<&str> = l.split(" ").collect();
            for w in words {
                let kv: Vec<&str> = w.split(":").collect();
                match kv.get(0) {
                    Some(k) => {
                        match kv.get(1) {
                            Some(v) => {
                                if k.len() > 0 && v.len() > 0 {
                                    current_passport.insert(k, v);
                                }
                            }, None => {}
                        }
                    }, None => {}
                }
            }
        } else if current_passport.len() > 0 {
            passports.push(current_passport);
            current_passport = HashMap::new();
        }
    }

    if current_passport.len() > 0 {
        passports.push(current_passport);
    }

    return passports
}
