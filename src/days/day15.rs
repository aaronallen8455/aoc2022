pub fn part_1(inp: &str) -> String {
    let lns: Vec<(C, C)> = inp
        .lines()
        .map(parse_ln).collect();
    let mut beacons = lns.iter().map(|x| x.1).filter(|x| x.1 == 2000000).map(|x| x.0).collect::<Vec<i32>>();
    beacons.dedup();
    let beacons = beacons.len();
    let mut rs = lns.into_iter()
        .filter_map(|l| range_at(l, 2000000))
        .collect();
    let rs = fold_ranges(&mut rs);
    (count(rs) - beacons as i32).to_string()
}

type C = (i32, i32);
type R = (i32, i32);
type Line = (C, C);

fn parse_ln(l: &str) -> Line {
    let mut i = l.split('=').skip(1);
    let x1 = i.next().unwrap().chars().take_while(|x| x.is_digit(10) || *x == '-').collect::<String>().parse().unwrap();
    let y1 = i.next().unwrap().chars().take_while(|x| x.is_digit(10) || *x == '-').collect::<String>().parse().unwrap();
    let x2 = i.next().unwrap().chars().take_while(|x| x.is_digit(10) || *x == '-').collect::<String>().parse().unwrap();
    let y2 = i.next().unwrap().chars().take_while(|x| x.is_digit(10) || *x == '-').collect::<String>().parse().unwrap();
    ((x1, y1), (x2, y2))
}

fn range_at(((sx, sy), (bx, by)): Line, l: i32) -> Option<R> {
    let dist = (sx - bx).abs() + (sy - by).abs();
    if (l - sy).abs() <= dist {
        let sp = dist - (l - sy).abs();
        Some((sx - sp, sx + sp))
    } else {
        None
    }
}

fn combine((a1, a2): R, (b1, b2): R) -> Option<R> {
    if (a1 <= b1 && a2 >= b1) || (b1 <= a1 && b2 >= a1) {
        Some((a1.min(b1), a2.max(b2)))
    } else { None }
}

fn fold_ranges(v: &mut Vec<R>) -> Vec<R> {
    let n = v.len();
    let mut fs = Vec::new();
    while let Some(x) = v.pop() {
        let mut new_v = Vec::new();
        let c = v.into_iter().fold(x, |c, &mut r| {
            match combine(c, r) {
                None => {
                    new_v.push(r);
                    c
                },
                Some(cc) => cc
            }
        });
        fs.push(c);
        v.clear();
        for e in new_v.into_iter() {
            v.push(e);
        }
    }
    if fs.len() != n {
        fold_ranges(&mut fs)
    } else { fs }
}

fn count(v: Vec<R>) -> i32 {
    let mut r = 0i32;
    for (a, b) in v.into_iter() {
        r += b - a + 1
    }
    r
}

pub fn part_2(inp: &str) -> String {
    let lns: Vec<(C, C)> = inp
        .lines()
        .map(parse_ln).collect();
    let (x, y) = search(&lns, (0, 0), 4000000, 4000000).unwrap();
    (x as i64 * 4000000 + y as i64).to_string()
}

fn search(v: &Vec<(C,C)>, c: C, w: i32, h: i32) -> Option<C> {
    if covered(v, c, w, h) {
        None
    } else if w == 0 && h == 0 {
        Some(c)
    } else {
        let hw = w / 2;
        let hh = h / 2;
        let tr = (c.0 + hw + 1, c.1);
        let bl = (c.0, c.1 + hh + 1);
        let br = (c.0 + hw + 1, c.1 + hh + 1);
        search(v, c, hw, hh)
            .or(search(v, tr, w - hw - 1, hh))
            .or(search(v, bl, hw, h - hh - 1))
            .or(search(v, br, w - hw - 1, h - hh - 1))
    }
}

fn covered(v: &Vec<(C, C)>, c: C, w: i32, h: i32) -> bool {
    let tl = c;
    let tr = (c.0 + w, c.1);
    let bl = (c.0, c.1 + h);
    let br = (c.0 + w, c.1 + h);
    for ((sx, sy), (bx, by)) in v.iter() {
        let dist = (sx - bx).abs() + (sy - by).abs();
        let in_rng = |(x, y)| {
            i32::abs(sx - x) + i32::abs(sy - y) <= dist
        };
        if in_rng(tl) && in_rng(tr) && in_rng(bl) && in_rng(br) {
            return true;
        }
    }
    false
}
