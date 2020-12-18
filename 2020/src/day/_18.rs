pub fn p1(s: &String) -> String {
    let v = parse(s);
    let mut total = 0;
    for problem in v {
        total += solve_problem(&problem);
    }
    return total.to_string();
}

pub fn p2(s: &String) -> String {
    let v = parse(s);
    let mut total = 0;
    for problem in v {
        total += solve_problem2(&problem);
    }
    return total.to_string();
}

fn solve_problem(v: &Vec<String>) -> u64 {
    let mut stack = vec![];
    let mut op = '+';
    let mut num = 0;

    for x in v {
        if x == "(" {
            stack.push((op, num));
            op = '+';
            num = 0;
        } else if x == "*" {
            op = '*';
        } else if x == "+" {
            op = '+';
        } else {
            let local_num = if x == ")" {
                let pair = stack.pop().unwrap();
                op = pair.0;
                pair.1
            } else {
                x.parse::<u64>().unwrap()
            };

            if op == '+' {
                num += local_num;
            } else if op == '*' {
                num *= local_num;
            }
        }
    }

    return num;
}

fn solve_problem2(v: &Vec<String>) -> u64 {
    let mut stack = vec![];
    let mut op = '+';
    let mut num = 0;

    for x in v {
        if x == "(" {
            stack.push((op, num));
            op = '+';
            num = 0;
        } else if x == "*" {
            stack.push(('*', num));
            op = '+';
            num = 0;
        } else if x == "+" {
            op = '+';
        } else {
            if x == ")" {

                loop {
                    let pair = stack.pop().unwrap();
                    op = pair.0;
                    if op == '*' {
                        num *= pair.1;
                        continue;
                    } else {
                        num += pair.1;
                        break;
                    }
                }
            } else {
                let local_num = x.parse::<u64>().unwrap();
                num += local_num;
            }
        }
    }

    for x in stack {
        num *= x.1;
    }

    return num;
}

fn parse(s: &String) -> Vec<Vec<String>> {
    return s.replace("(", "( ")
        .replace(")", " )")
        .lines()
        .map(|x| x.to_string().split(" ").map(|y| y.to_string()).collect())
        .collect();
}
