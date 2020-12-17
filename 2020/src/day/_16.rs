use regex::Regex;
use std::collections::HashSet;

pub fn p1(s: &String) -> String {
    let (fields, _, nearby_tickets) = parse(s);

    let mut error_rate = 0;
    for ticket in nearby_tickets {
        for ticket_field in ticket {
            let is_in_valid_range = fields.iter().any(|x| (ticket_field >= x.range1.0 && ticket_field <= x.range1.1) || (ticket_field >= x.range2.0 && ticket_field <= x.range2.1));
            if !is_in_valid_range {
                error_rate += ticket_field;
            }
        }
    }

    return error_rate.to_string();
}

pub fn p2(s: &String) -> String {
    let (fields, your_ticket, nearby_tickets) = parse(s);

    let nearby_tickets: Vec<&Vec<u32>> = nearby_tickets.iter()
        .filter(|ticket| !ticket.iter()
                .any(|ticket_field| !fields.iter()
                     .any(|x| (ticket_field >= &x.range1.0 && ticket_field <= &x.range1.1) || (ticket_field >= &x.range2.0 && ticket_field <= &x.range2.1))
                    )).collect();

    let mut possible_fields_for_column = vec![];
    for i in 0..fields.len() {
        possible_fields_for_column.push(get_possible_fields(&fields, &nearby_tickets, i));
    }

    let mut set_vals = HashSet::new();
    loop {
        for values in possible_fields_for_column.iter_mut() {
            if values.len() == 1 {
                set_vals.insert(*values.iter().next().unwrap());
            } else {
                *values = values.difference(&set_vals).into_iter().map(|x| *x).collect();
            }
        }

        if possible_fields_for_column.iter().all(|x| x.len() == 1) {
            break;
        }
    }

    let col_index_to_field: Vec<usize> = possible_fields_for_column.iter().map(|x| *x.iter().next().unwrap()).collect();
    let mut answer: u64 = 1;
    for (i, val) in your_ticket.iter().enumerate() {
        let field = &fields[col_index_to_field[i]];
        if field.name.starts_with("departure ") {
            answer *= *val as u64;
        }
    }

    return answer.to_string();
}

fn get_possible_fields(fields: &Vec<Field>, tickets: &Vec<&Vec<u32>>, pos: usize) -> HashSet<usize> {
    let mut possible_fields = HashSet::new();
    for (i, field) in fields.iter().enumerate() {
        let is_in_tickets = tickets.iter()
            .map(|x| x[pos])
            .all(|val| val >= field.range1.0 && val <= field.range1.1 || val >= field.range2.0 && val <= field.range2.1);

        if is_in_tickets {
            possible_fields.insert(i);
        }
    }

    return possible_fields;
}

#[derive(Debug)]
struct Field {
    name: String,
    range1: (u32, u32),
    range2: (u32, u32),
}

fn parse(s: &String) -> (Vec<Field>, Vec<u32>, Vec<Vec<u32>>) {
    let mut fields = vec![];
    let mut your_ticket = vec![];
    let mut nearby_tickets = vec![];

    let field_re = Regex::new(r"^(.+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
    let csv_re   = Regex::new(r"^\d+(,\d+)*$").unwrap();

    for l in s.lines() {
        if field_re.is_match(l) {
            let cap = field_re.captures(l).unwrap();
            fields.push(Field { name: cap[1].to_string(), range1: (cap[2].parse::<u32>().unwrap(), cap[3].parse::<u32>().unwrap()), range2: (cap[4].parse::<u32>().unwrap(), cap[5].parse::<u32>().unwrap()) });
        } else if your_ticket.len() == 0 && csv_re.is_match(l) {
            your_ticket = l.split(",").map(|x| x.parse::<u32>().unwrap()).collect();
        } else if csv_re.is_match(l) {
            nearby_tickets.push(l.split(",").map(|x| x.parse::<u32>().unwrap()).collect());
        }
    }

    return (fields, your_ticket, nearby_tickets);
}
