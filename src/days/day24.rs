use std::collections::HashSet;

pub fn part_1(inp: &str) -> String {
    let mut m = parse_map(inp);
    let mut r = 1;
    let mut cs = HashSet::from([(-1,0)]);
    let target = (m.len() as i32, m[0].len() as i32 - 1);
    mv_bs(&mut m);
    while !mv_cs(&m, &mut cs, &target) {
        mv_bs(&mut m);
        r += 1;
    }
    r.to_string()
}

type C = (i32, i32);
type Map = Vec<Vec<Vec<Dir>>>;
type Cands = HashSet<C>;

fn parse_map(i: &str) -> Map {
    let mut m: Map = i.lines()
        .map(|r| {
            let mut rv:Vec<Vec<Dir>> = r.chars().map(parse_cell).collect();
            rv.pop();
            rv.remove(0);
            rv
        })
        .collect();
    m.pop();
    m.remove(0);
    m
}

fn parse_cell(c: char) -> Vec<Dir> {
    match c {
        '^' => vec![Dir::N],
        'v' => vec![Dir::S],
        '<' => vec![Dir::W],
        '>' => vec![Dir::E],
        _ => vec![]
    }
}

#[derive(Clone, Debug)]
enum Dir {
    N,
    S,
    E,
    W,
}

fn mv_cs(m: &Map, cs: &mut Cands, target: &C) -> bool {
    let mut new_cs: Cands = HashSet::new();
    for &(r, c) in cs.iter() {
        for coord@(nr, nc) in [(r-1,c), (r+1,c), (r,c-1), (r,c+1), (r,c)] {
            if coord == *target {
                return true;
            }
            if nr == -1 && nc == 0 {
                new_cs.insert((nr, nc));
                continue;
            }
            if nr == m.len() as i32 && nc == m[0].len() as i32 - 1 {
                new_cs.insert(coord);
                continue;
            }

            if nr < 0 { continue; }
            if nr >= m.len() as i32 { continue; }
            if nc >= m[0].len() as i32 { continue; }
            if nc < 0 { continue; }
            if m[nr as usize][nc as usize].len() > 0 { continue; }
            new_cs.insert(coord);
        }
    }
    *cs = new_cs;
    false
}

fn mv_bs(m: &mut Map) {
    let mut vs = Vec::new();
    for (ri, r) in m.into_iter().enumerate() {
        for (ci, dirs) in r.into_iter().enumerate() {
            while let Some(d) = dirs.pop() {
                vs.push((d.clone(), match d {
                    Dir::N => (ri as i32 - 1, ci as i32),
                    Dir::S => (ri as i32 + 1, ci as i32),
                    Dir::E => (ri as i32, ci as i32 + 1),
                    Dir::W => (ri as i32, ci as i32 - 1),

                }));
            }
        }
    }
    for (d, mut c) in vs {
        wrap(m, &mut c);
        m[c.0 as usize][c.1 as usize].push(d);
    }
}

fn wrap(m: &Map, c: &mut C) {
    if c.0 < 0 { c.0 = m.len() as i32 - 1 }
    else if c.0 >= m.len() as i32 { c.0 = 0 }
    else if c.1 < 0 { c.1 = m[0].len() as i32 - 1}
    else if c.1 >= m[0].len() as i32 { c.1 = 0 }
}

pub fn part_2(inp: &str) -> String {
    let mut m = parse_map(inp);
    let mut r = 1;
    let target = (m.len() as i32, m[0].len() as i32 - 1);
    let mut cs = HashSet::from([(-1,0)]);
    mv_bs(&mut m);
    while !mv_cs(&m, &mut cs, &target) {
        mv_bs(&mut m);
        r += 1;
    }
    let target = (-1, 0);
    let mut cs = HashSet::from([(m.len() as i32,m[0].len() as i32 - 1)]);
    mv_bs(&mut m);
    r += 1;
    while !mv_cs(&m, &mut cs, &target) {
        mv_bs(&mut m);
        r += 1;
    }
    let target = (m.len() as i32, m[0].len() as i32 - 1);
    let mut cs = HashSet::from([(-1, 0)]);
    mv_bs(&mut m);
    r += 1;
    while !mv_cs(&m, &mut cs, &target) {
        mv_bs(&mut m);
        r += 1;
    }

    r.to_string()
}
