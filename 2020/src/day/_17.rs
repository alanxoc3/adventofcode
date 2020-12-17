use std::collections::HashSet;

pub fn p1(s: &String) -> String {
    let mut hs = parse_3d(s);

    for _ in 0..6 {
        let mut dhs = HashSet::new();
        let b = get_space_bounds_3d(&hs);
        for x in b.xmin..=b.xmax {
            for y in b.ymin..=b.ymax {
                for z in b.zmin..=b.zmax {
                    let pos = (x, y, z);
                    let active = hs.contains(&pos);
                    let neighbors = count_neighbors_3d(&hs, &pos);
                    if active && (neighbors == 2 || neighbors == 3) || !active && neighbors == 3 {
                        dhs.insert(pos);
                    }
                }
            }
        }
        hs = dhs;
    }

    return hs.len().to_string();
}

pub fn p2(s: &String) -> String {
    let mut hs = parse_4d(s);

    for _ in 0..6 {
        let mut dhs = HashSet::new();
        let b = get_space_bounds_4d(&hs);
        for w in b.wmin..=b.wmax {
            for x in b.xmin..=b.xmax {
                for y in b.ymin..=b.ymax {
                    for z in b.zmin..=b.zmax {
                        let pos = (w, x, y, z);
                        let active = hs.contains(&pos);
                        let neighbors = count_neighbors_4d(&hs, &pos);
                        if active && (neighbors == 2 || neighbors == 3) || !active && neighbors == 3 {
                            dhs.insert(pos);
                        }
                    }
                }
            }
        }
        hs = dhs;
    }

    return hs.len().to_string();
}

// ------------------------------------------------ 3d code ------------------------------------------------
#[derive(Debug)]
struct Bounds3d {
    xmin: i32, xmax: i32,
    ymin: i32, ymax: i32,
    zmin: i32, zmax: i32,
}

fn get_space_bounds_3d(hs: &HashSet<(i32,i32,i32)>) -> Bounds3d {
    let item = *hs.iter().next().unwrap();
    let (mut xmin, mut xmax, mut ymin, mut ymax, mut zmin, mut zmax) = (item.0, item.0, item.1, item.1, item.2, item.2);

    for h in hs {
        if xmin > h.0 { xmin = h.0 }
        if xmax < h.0 { xmax = h.0 }
        if ymin > h.1 { ymin = h.1 }
        if ymax < h.1 { ymax = h.1 }
        if zmin > h.2 { zmin = h.2 }
        if zmax < h.2 { zmax = h.2 }
    }

    return Bounds3d {
        xmin: xmin-1, xmax: xmax+1,
        ymin: ymin-1, ymax: ymax+1,
        zmin: zmin-1, zmax: zmax+1,
    };
}

fn count_neighbors_3d(hs: &HashSet<(i32,i32,i32)>, pos: &(i32, i32, i32)) -> usize {
    let set: HashSet<(i32, i32, i32)> = vec![
        (-1, -1, -1), (-1, -1,  0), (-1, -1,  1), (-1,  0, -1), (-1,  0,  0), (-1,  0,  1), (-1,  1, -1), (-1,  1,  0), (-1,  1,  1),
        ( 0, -1, -1), ( 0, -1,  0), ( 0, -1,  1), ( 0,  0, -1),               ( 0,  0,  1), ( 0,  1, -1), ( 0,  1,  0), ( 0,  1,  1),
        ( 1, -1, -1), ( 1, -1,  0), ( 1, -1,  1), ( 1,  0, -1), ( 1,  0,  0), ( 1,  0,  1), ( 1,  1, -1), ( 1,  1,  0), ( 1,  1,  1),
    ].iter().map(|p| (p.0+pos.0, p.1+pos.1, p.2+pos.2)).collect();

    return set.intersection(hs).into_iter().count();
}

fn parse_3d(s: &String) -> HashSet<(i32,i32,i32)> {
    let mut hs = HashSet::new();

    for (y, l) in s.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            if c == '#' {
                hs.insert((x as i32, y as i32, 0));
            }
        }
    }

    return hs;
}

// ------------------------------------------------ 4d code ------------------------------------------------
#[derive(Debug)]
struct Bounds4d {
    wmin: i32, wmax: i32,
    xmin: i32, xmax: i32,
    ymin: i32, ymax: i32,
    zmin: i32, zmax: i32,
}

fn get_space_bounds_4d(hs: &HashSet<(i32,i32,i32,i32)>) -> Bounds4d {
    let item = *hs.iter().next().unwrap();
    let (mut wmin, mut wmax, mut xmin, mut xmax, mut ymin, mut ymax, mut zmin, mut zmax) = (item.0, item.0, item.1, item.1, item.2, item.2, item.3, item.3);

    for h in hs {
        if wmin > h.0 { wmin = h.0 }
        if wmax < h.0 { wmax = h.0 }
        if xmin > h.1 { xmin = h.1 }
        if xmax < h.1 { xmax = h.1 }
        if ymin > h.2 { ymin = h.2 }
        if ymax < h.2 { ymax = h.2 }
        if zmin > h.3 { zmin = h.3 }
        if zmax < h.3 { zmax = h.3 }
    }

    return Bounds4d {
        wmin: wmin-1, wmax: wmax+1,
        xmin: xmin-1, xmax: xmax+1,
        ymin: ymin-1, ymax: ymax+1,
        zmin: zmin-1, zmax: zmax+1,
    };
}

fn count_neighbors_4d(hs: &HashSet<(i32,i32,i32,i32)>, pos: &(i32, i32, i32, i32)) -> usize {
    let set: HashSet<(i32, i32, i32, i32)> = vec![
        (-1, -1, -1, -1), (-1, -1, -1,  0), (-1, -1, -1,  1), (-1, -1,  0, -1), (-1, -1,  0,  0), (-1, -1,  0,  1), (-1, -1,  1, -1), (-1, -1,  1,  0), (-1, -1,  1,  1),
        (-1,  0, -1, -1), (-1,  0, -1,  0), (-1,  0, -1,  1), (-1,  0,  0, -1), (-1,  0,  0,  0), (-1,  0,  0,  1), (-1,  0,  1, -1), (-1,  0,  1,  0), (-1,  0,  1,  1),
        (-1,  1, -1, -1), (-1,  1, -1,  0), (-1,  1, -1,  1), (-1,  1,  0, -1), (-1,  1,  0,  0), (-1,  1,  0,  1), (-1,  1,  1, -1), (-1,  1,  1,  0), (-1,  1,  1,  1),

        ( 0, -1, -1, -1), ( 0, -1, -1,  0), ( 0, -1, -1,  1), ( 0, -1,  0, -1), ( 0, -1,  0,  0), ( 0, -1,  0,  1), ( 0, -1,  1, -1), ( 0, -1,  1,  0), ( 0, -1,  1,  1),
        ( 0,  0, -1, -1), ( 0,  0, -1,  0), ( 0,  0, -1,  1), ( 0,  0,  0, -1),                   ( 0,  0,  0,  1), ( 0,  0,  1, -1), ( 0,  0,  1,  0), ( 0,  0,  1,  1),
        ( 0,  1, -1, -1), ( 0,  1, -1,  0), ( 0,  1, -1,  1), ( 0,  1,  0, -1), ( 0,  1,  0,  0), ( 0,  1,  0,  1), ( 0,  1,  1, -1), ( 0,  1,  1,  0), ( 0,  1,  1,  1),

        ( 1, -1, -1, -1), ( 1, -1, -1,  0), ( 1, -1, -1,  1), ( 1, -1,  0, -1), ( 1, -1,  0,  0), ( 1, -1,  0,  1), ( 1, -1,  1, -1), ( 1, -1,  1,  0), ( 1, -1,  1,  1),
        ( 1,  0, -1, -1), ( 1,  0, -1,  0), ( 1,  0, -1,  1), ( 1,  0,  0, -1), ( 1,  0,  0,  0), ( 1,  0,  0,  1), ( 1,  0,  1, -1), ( 1,  0,  1,  0), ( 1,  0,  1,  1),
        ( 1,  1, -1, -1), ( 1,  1, -1,  0), ( 1,  1, -1,  1), ( 1,  1,  0, -1), ( 1,  1,  0,  0), ( 1,  1,  0,  1), ( 1,  1,  1, -1), ( 1,  1,  1,  0), ( 1,  1,  1,  1),
    ].iter().map(|p| (p.0+pos.0, p.1+pos.1, p.2+pos.2, p.3+pos.3)).collect();

    return set.intersection(hs).into_iter().count();
}

fn parse_4d(s: &String) -> HashSet<(i32,i32,i32,i32)> {
    let mut hs = HashSet::new();

    for (y, l) in s.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            if c == '#' {
                hs.insert((0, x as i32, y as i32, 0));
            }
        }
    }

    return hs;
}
