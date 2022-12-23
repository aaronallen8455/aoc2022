use std::collections::{HashSet, VecDeque, HashMap};

pub fn part_1(inp: &str) -> String {
    let mut g = parse_g(inp);
    let mut ds = VecDeque::from([Dir::N, Dir::S, Dir::W, Dir::E]);
    for _ in 0..10 {
        turn(&mut g, &mut ds, &mut false);
    }
    count(&g).to_string()
}

fn count(g: &Grid) -> u32 {
    let mut res = 0;
    let mut max_r = -999999999;
    let mut min_r = 999999999;
    let mut max_c = -999999999;
    let mut min_c = 999999999;
    for (r, c) in g {
        max_r = max_r.max(*r);
        min_r = min_r.min(*r);
        max_c = max_c.max(*c);
        min_c = min_c.min(*c);
    }
    for r in min_r..=max_r {
        for c in min_c..=max_c {
            if !g.contains(&(r, c)) {
                res += 1;
            }
        }
    }
    res
}

fn parse_g(i: &str) -> Grid {
    let mut g = HashSet::new();
    for (r, t) in i.lines().enumerate() {
        for (c, ch) in t.chars().enumerate() {
            if ch == '#' {
                g.insert((r as i32, c as i32));
            }
        }
    }
    g
}

enum Dir {
    N,
    S,
    E,
    W,
}

type C = (i32, i32);

type Grid = HashSet<C>;

fn turn(g: &mut Grid, ds: &mut VecDeque<Dir>, moved: &mut bool) {
    let mut cons: HashMap<C, Option<C>> = HashMap::new();
    for c in g.iter() {
        if empty(c, g) { continue }
        let mut ocon = None;
        for d in ds.iter() {
            ocon = ocon.or(consider(c, d, g))
        }
        match ocon {
            None => (),
            Some(con) => {
                cons.entry(con).and_modify(|x| *x = None).or_insert(Some(*c));
            },
        }
    }
    *moved = false;
    for (k, v) in cons.into_iter() {
        match v {
            None => (),
            Some(v) => {
                g.remove(&v);
                g.insert(k);
                *moved = true;
            }
        }
    }

    ds.rotate_left(1);
}

fn consider((r, c): &C, d: &Dir, g: &Grid) -> Option<C> {
    match d {
        Dir::N if ![(r-1, c-1), (r-1, *c), (r-1, c+1)].iter()
            .any(|x| g.contains(x)) => Some((r-1, *c)),
        Dir::S if ![(r+1, c-1), (r+1, *c), (r+1, c+1)].iter()
            .any(|x| g.contains(x)) => Some((r+1, *c)),
        Dir::E if ![(r-1, c+1), (*r, c+1), (r+1, c+1)].iter()
            .any(|x| g.contains(x)) => Some((*r, c+1)),
        Dir::W if ![(r-1, c-1), (*r, c-1), (r+1, c-1)].iter()
            .any(|x| g.contains(x)) => Some((*r, c-1)),
        _ => None
    }
}

fn empty(coord@(r, c): &C, g: &Grid) -> bool {
    for rr in r-1..=r+1 {
        for cc in c-1..=c+1 {
            if (rr, cc) == *coord { continue }
            if g.contains(&(rr,cc)) { return false }
        }
    }
    true
}

pub fn part_2(inp: &str) -> String {
    let mut g = parse_g(inp);
    let mut ds = VecDeque::from([Dir::N, Dir::S, Dir::W, Dir::E]);
    let mut moved = true;
    let mut count = 0;
    while moved {
        count += 1;
        turn(&mut g, &mut ds, &mut moved);
    }
    count.to_string()
}
