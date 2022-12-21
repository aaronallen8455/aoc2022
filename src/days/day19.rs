pub fn part_1(inp: &str) -> String {
    inp.lines()
        .map(parse_ln)
        .map(geos)
        .enumerate()
        .map(|(i, x)| (i + 1) as u32 * x)
        .sum::<u32>()
        .to_string()
}

fn geos(bp: Blue) -> u32 {
    let mut v = vec![St { ore: 0, clay: 0, obsid: 0, geo: 0, orer: 1, clayr: 0, obsidr: 0, geor: 0 }];
    for _ in 0..24 {
        let new: Vec<St> = v.iter().map(|x| tick(&bp, &x)).collect::<Vec<Vec<St>>>().concat();
        v = new;
    }
    v.iter().map(|x| x.geo).max().unwrap()
}

fn tick(bp: &Blue, st: &St) -> Vec<St> {
    let new = St {
        ore: st.ore + st.orer,
        clay: st.clay + st.clayr,
        obsid: st.obsid + st.obsidr,
        geo: st.geo + st.geor,
        ..*st
    };
    let mut r = Vec::new();
    if new.clayr == 0 {
        let mut s = new.clone();
        if s.buy_orer(st, &bp.orer) {
            r.push(s);
        }

        let mut s = new.clone();
        if s.buy_clayr(st, &bp.clayr) {
            r.push(s);
        } else {
            r.push(new.clone());
        }
    } else if new.obsidr == 0 {
        let mut s = new.clone();
        if s.buy_clayr(st, &bp.clayr) {
            r.push(s);
        }

        let mut s = new.clone();
        if s.buy_obsidr(st, &bp.obsidr) {
            r.push(s);
        } else {
            r.push(new.clone());
        }
    } else if new.geor == 0 {
        let mut s = new.clone();
        if s.buy_obsidr(st, &bp.obsidr) {
            r.push(s);
        }

        let mut s = new.clone();
        if s.buy_geodr(st, &bp.geor) {
            r.push(s);
        } else {
            r.push(new.clone());
        }
    } else {
        let mut s = new.clone();
        if s.buy_geodr(st, &bp.geor) {
            r.push(s);
        } else {
            r.push(new.clone());
        }
    }
    if r.len() == 0 {
        r.push(new);
    }
    r
}

#[derive(Debug)]
struct Cost {
    ore: u32,
    clay: u32,
    obsid: u32,
}

#[derive(Debug)]
struct Blue {
    orer: Cost,
    clayr: Cost,
    obsidr: Cost,
    geor: Cost,
}

#[derive(Clone, Debug)]
struct St {
    ore: u32,
    clay: u32,
    obsid: u32,
    geo: u32,
    orer: u32,
    clayr: u32,
    obsidr: u32,
    geor: u32,
}

impl St {
    fn buy_orer(&mut self, cur: &St, c: &Cost) -> bool {
        if cur.ore >= c.ore {
            self.ore -= c.ore;
            self.orer += 1;
            true
        } else { false }
    }
    fn buy_clayr(&mut self, cur: &St, c: &Cost) -> bool {
        if cur.ore >= c.ore {
            self.ore -= c.ore;
            self.clayr += 1;
            true
        } else { false }
    }
    fn buy_obsidr(&mut self, cur: &St, c: &Cost) -> bool {
        if cur.ore >= c.ore && cur.clay >= c.clay {
            self.ore -= c.ore;
            self.clay -= c.clay;
            self.obsidr += 1;
            true
        } else { false }
    }
    fn buy_geodr(&mut self, cur: &St, c: &Cost) -> bool {
        if cur.ore >= c.ore && cur.obsid >= c.obsid {
            self.ore -= c.ore;
            self.obsid -= c.obsid;
            self.geor += 1;
            true
        } else { false }
    }
}

fn parse_ln(i: &str) -> Blue {
    let mut it = i.split_whitespace();
    it.next();
    it.next();
    it.next();
    it.next();
    it.next();
    it.next();
    let orer = it.next().unwrap().parse().unwrap();
    it.next();
    it.next();
    it.next();
    it.next();
    it.next();
    let clayr = it.next().unwrap().parse().unwrap();
    it.next();
    it.next();
    it.next();
    it.next();
    it.next();
    let obs1 = it.next().unwrap().parse().unwrap();
    it.next();
    it.next();
    let obs2 = it.next().unwrap().parse().unwrap();
    it.next();
    it.next();
    it.next();
    it.next();
    it.next();
    let geo1 = it.next().unwrap().parse().unwrap();
    it.next();
    it.next();
    let geo2 = it.next().unwrap().parse().unwrap();
    Blue {
        orer: Cost {
            ore: orer,
            clay: 0,
            obsid: 0,
        },
        clayr: Cost {
            ore: clayr,
            clay: 0,
            obsid: 0,
        },
        obsidr: Cost {
            ore: obs1,
            clay: obs2,
            obsid: 0,
        },
        geor: Cost {
            ore: geo1,
            clay: 0,
            obsid: geo2,
        }
    }
}

pub fn part_2(_inp: &str) -> String {
    String::new()
}
