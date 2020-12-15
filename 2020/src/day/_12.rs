use regex::Regex;

pub fn p1(s: &String) -> String {
    let v = parse(s);

    let mut dir = 0;
    let mut x = 0;
    let mut y = 0;

    for cmd in v {
        dir = match cmd.0.as_str() {
            "L" => { dir + cmd.1 },
            "R" => { dir + (360 - cmd.1) },
            _ => dir,
        } % 360;

        let cur_dir = match cmd.0.as_str() {
            "E" => { 0 },
            "N" => { 90 },
            "W" => { 180 },
            "S" => { 270 },
            _   => { dir },
        };

        let dist = match cmd.0.as_str() {
            "E" | "N" | "W" | "S" | "F" => { cmd.1 },
            "L" | "R" | _ => { 0 },
        };

        match cur_dir {
            0 =>   { x += dist },
            90 =>  { y += dist },
            180 => { x -= dist },
            270 => { y -= dist },
            _ => { println!("prob with dat puzzle") },
        }
    }

    return (x.abs() + y.abs()).to_string();
}

pub fn p2(s: &String) -> String {
    let v = parse(s);

    let mut x = 0;
    let mut y = 0;
    let mut wx = 10;
    let mut wy = 1;

    for cmd in v {
        let mut dir = 0;

        match cmd.0.as_str() {
            "E" => { wx += cmd.1; },
            "N" => { wy += cmd.1; },
            "W" => { wx -= cmd.1; },
            "S" => { wy -= cmd.1; },
            "F" => {
                x += wx * cmd.1;
                y += wy * cmd.1;
            },
            "L" => {
                dir = cmd.1;
            },
            "R" => {
                dir = 360 - cmd.1;
            },
            _ => {},
        };

        let (dwx, dwy) = (wx, wy);
        match dir {
            0 =>  {},
            90 =>  {
                wx = -dwy;
                wy = dwx;
            },
            180 => {
                wx = -dwx;
                wy = -dwy;
            },
            270 => {
                wx = dwy;
                wy = -dwx;
            },
            _   => { println!("problem with puzzle input"); },
        };
    }

    return (x.abs() + y.abs()).to_string();
}

fn parse(s: &String) -> Vec<(String, i32)> {
    let re = Regex::new(r"^(\w)(\d+)$").unwrap();
    let mut v = vec![];

    for l in s.lines() {
        let cap = re.captures(l).unwrap();
        v.push((cap[1].to_string(), cap[2].parse::<i32>().unwrap()));
    }

    return v;
}
