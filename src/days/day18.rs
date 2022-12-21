use std::collections::HashSet;

pub fn part_1(inp: &str) -> String {
    let coords: HashSet<(i32, i32, i32)> =
        HashSet::from_iter(inp.lines().map(parse_ln));
    count(coords).to_string()
}

fn parse_ln(i: &str) -> (i32, i32, i32) {
    let v: Vec<i32> = i.split(',').map(|x| x.parse().unwrap()).collect();
    (v[0], v[1], v[2])
}

fn count(s: HashSet<(i32, i32, i32)>) -> i32 {
    let mut r = 0;
    for (x, y, z) in s.iter() {
        if !s.contains(&(*x+1,*y,*z)) {
            r += 1;
        }
        if !s.contains(&(*x-1,*y,*z)) {
            r += 1;
        }
        if !s.contains(&(*x,*y+1,*z)) {
            r += 1;
        }
        if !s.contains(&(*x,*y-1,*z)) {
            r += 1;
        }
        if !s.contains(&(*x,*y,*z+1)) {
            r += 1;
        }
        if !s.contains(&(*x,*y,*z-1)) {
            r += 1;
        }
    }
    r
}

pub fn part_2(inp: &str) -> String {
    let coords: HashSet<(i32, i32, i32)> =
        HashSet::from_iter(inp.lines().map(parse_ln));
    count(fill(coords)).to_string()
}

fn fill(s: HashSet<(i32, i32, i32)>) -> HashSet<(i32, i32, i32)> {
    let (mx, my, mz) = s.iter().fold((0, 0, 0), |(mx, my, mz), (x, y, z)| {
        (mx.max(*x), my.max(*y), mz.max(*z))
    });
    let out = flood(&s, mx, my, mz);
    let mut r = HashSet::new();
    for x in 0..=mx {
        for y in 0..=my {
            for z in 0..=mz {
                if !out.contains(&(x,y,z)) {
                    r.insert((x,y,z));
                }
            }
        }
    }
    r
}

fn flood(s: &HashSet<(i32, i32, i32)>, mx: i32, my: i32, mz: i32) -> HashSet<(i32, i32, i32)> {
    let mut v = Vec::from([(0,0,0)]);
    let mut out = HashSet::new();
    while v.len() > 0 {
        let c@(x,y,z) = v.pop().unwrap();
        if !out.contains(&c) && !s.contains(&c) {
            out.insert((x,y,z));
            if x < mx { v.push((x+1,y,z)) }
            if x > 0  { v.push((x-1,y,z)) }
            if y < my { v.push((x,y+1,z)) }
            if y > 0  { v.push((x,y-1,z)) }
            if z < mz { v.push((x,y,z+1)) }
            if z > 0  { v.push((x,y,z-1)) }
        }
    }
    out
}
