pub fn part_1(inp: &str) -> String {
    let mut i = inp.split("\n\n");
    let mstr = i.next().unwrap();
    let map: Map = mstr.lines().map(parse_ln).collect();
    let istr = i.next().unwrap();
    let instr: Vec<Instr> = parse_instr(istr);
    let mut dir = Dir::R;
    let mut coord = (0, map[0].iter().take_while(|t| **t == Tile::E || **t == Tile::W).count() as i32);
    for i in instr {
        go(&map, &mut dir, &i, &mut coord);
    }
    (1000 * (coord.0 + 1) + 4 * (coord.1 + 1) + dir_num(dir)).to_string()
}

fn go(m: &Map, d: &mut Dir, i: &Instr, c: &mut C) {
    match i {
        Instr::Go(dist) => *c = mv(m, d, *dist, c),
        Instr::T(t) => *d = turn(t, d)
    }
}

fn parse_ln(i: &str) -> Vec<Tile> {
    i.chars().map(|c| {
        match c {
            ' ' => Tile::E,
            '.' => Tile::O,
            '#' => Tile::W,
            _ => Tile::E,
        }
    }).collect()
}

fn parse_instr(i: &str) -> Vec<Instr> {
    let mut r = Vec::new();
    let mut num = String::new();
    for c in i.chars() {
        if c.is_digit(10) {
            num.push(c);
        } else {
            r.push(Instr::Go(num.parse().unwrap()));
            num.clear();
            match c {
                'L' => r.push(Instr::T(Turn::L)),
                'R' => r.push(Instr::T(Turn::R)),
                _ => (),
            }
        }
    }
    if num.len() > 0 {
        r.push(Instr::Go(num.parse().unwrap()));
    }
    r
}

type C = (i32, i32);
type Map = Vec<Vec<Tile>>;

#[derive(PartialEq)]
enum Tile {
    E, O, W,
}

#[derive(Debug)]
enum Turn {
    L,
    R,
}

#[derive(Debug, PartialEq, Clone)]
enum Dir {
    L,
    R,
    U,
    D
}

fn dir_num(d: Dir) -> i32 {
    match d {
        Dir::R => 0,
        Dir::D => 1,
        Dir::L => 2,
        Dir::U => 3,
    }
}

fn turn(t: &Turn, d: &Dir) -> Dir {
    match (t, d) {
        (Turn::L, Dir::L) => Dir::D,
        (Turn::R, Dir::L) => Dir::U,
        (Turn::L, Dir::R) => Dir::U,
        (Turn::R, Dir::R) => Dir::D,
        (Turn::L, Dir::U) => Dir::L,
        (Turn::R, Dir::U) => Dir::R,
        (Turn::L, Dir::D) => Dir::R,
        (Turn::R, Dir::D) => Dir::L,
    }
}

fn step(d: &Dir, (r, c): &mut C) {
    match d {
        Dir::L => *c -= 1,
        Dir::R => *c += 1,
        Dir::U => *r -= 1,
        Dir::D => *r += 1,
    }
}

fn wrap(m: &Map, d: &Dir, coord@(r, c) : C) -> C {
    let out_or_empty = |row: &&Vec<Tile>| {
        c >= row.len() as i32 || row[c as usize] == Tile::E
    };
    if r < 0 || c < 0 || r >= m.len() as i32 || c >= m[r as usize].len() as i32 || m[r as usize][c as usize] == Tile::E {
        match d {
            Dir::L => (r, m[r as usize].iter().rev().skip_while(|t| **t == Tile::E).count() as i32 - 1),
            Dir::R => (r, m[r as usize].iter().take_while(|t| **t == Tile::E).count() as i32),
            Dir::U => (m.iter().rev().skip_while(out_or_empty).count() as i32 - 1, c),
            Dir::D => (m.iter().take_while(out_or_empty).count() as i32, c),
        }
    } else { coord }
}

fn wrap2(d: &mut Dir, coord: C) -> C {
    match coord {
        // l
        (200, c) if *d == Dir::D => (0, c + 100),
        (-1, c) if c > 99 && *d == Dir::U => (199, c - 100),
        // k
        (r, -1) if r > 149 && *d == Dir::L => {
            *d = Dir::D;
            (0, r - 100)
        },
        (-1, c) if c > 49 && c < 100 && *d == Dir::U => {
            *d = Dir::R;
            (c + 100, 0)
        },
        // j
        (r, 50) if r > 149 && *d == Dir::R => {
            *d = Dir::U;
            (149, r - 100)
        },
        (150, c) if c > 49 && *d == Dir::D => {
            *d = Dir::L;
            (c + 100, 49)
        },
        // g
        (r, -1) if r > 99 && r < 150 && *d == Dir::L => {
            *d = Dir::R;
            (49 - (r - 100), 50)
        },
        (r, 49) if r < 50 && *d == Dir::L => {
            *d = Dir::R;
            (49 - r + 100, 0)
        },
        // e
        (99, c) if c < 50 && *d == Dir::U => {
            *d = Dir::R;
            (c + 50, 50)
        },
        (r, 49) if r > 49 && r < 100 && *d == Dir::L => {
            *d = Dir::D;
            (100, r - 50)
        }
        // d
        (r, 100) if r > 99 && r < 150 && *d == Dir::R => {
            *d = Dir::L;
            (49 - (r - 100), 149)
        },
        (r, 150) if *d == Dir::R => {
            *d = Dir::L;
            (49 - r + 100, 99)
        },
        // c
        (r, 100) if r > 49 && r < 100 && *d == Dir::R => {
            *d = Dir::U;
            (49, r + 50)
        },
        (50, c) if c > 99 && *d == Dir::D => {
            *d = Dir::L;
            (c - 50, 99)
        },
        _ => coord
    }
}

fn mv(m: &Map, d: &Dir, dist: i32, (r, c): &C) -> C {
    let mut rc = (*r, *c);
    for _ in 0..dist {
        let before = rc;
        step(d, &mut rc);
        rc = wrap(m, d, rc);
        if m[rc.0 as usize][rc.1 as usize] == Tile::W {
            return before;
        }
    }
    rc
}

#[derive(Debug)]
enum Instr {
    T(Turn),
    Go(i32),
}

pub fn part_2(inp: &str) -> String {
    let mut i = inp.split("\n\n");
    let mstr = i.next().unwrap();
    let map: Map = mstr.lines().map(parse_ln).collect();
    let istr = i.next().unwrap();
    let instr: Vec<Instr> = parse_instr(istr);
    let mut dir = Dir::R;
    let mut coord = (0, map[0].iter().take_while(|t| **t == Tile::E || **t == Tile::W).count() as i32);
    for i in instr {
        go2(&map, &mut dir, &i, &mut coord);
    }
    (1000 * (coord.0 + 1) + 4 * (coord.1 + 1) + dir_num(dir)).to_string()
}

fn go2(m: &Map, d: &mut Dir, i: &Instr, c: &mut C) {
    match i {
        Instr::Go(dist) => *c = mv2(m, d, *dist, c),
        Instr::T(t) => *d = turn(t, d)
    }
}

fn mv2(m: &Map, d: &mut Dir, dist: i32, (r, c): &C) -> C {
    let mut rc = (*r, *c);
    for _ in 0..dist {
        let before_c = rc;
        let before_d = d.clone();
        step(d, &mut rc);
        rc = wrap2(d, rc);
        if m[rc.0 as usize][rc.1 as usize] == Tile::W {
            *d = before_d;
            return before_c;
        }
    }
    rc
}
