pub fn part_1(inp: &str) -> String {
    inp.lines()
        .map(parse_ln)
        .map(|x| geos(24, x))
        .enumerate()
        .map(|(i, x)| (i + 1) as u32 * x)
        .sum::<u32>()
        .to_string()
}

fn geos(n: usize, bp: Blue) -> u32 {
    let mut v = vec![St { ore: 0, clay: 0, obsid: 0, geo: 0, orer: 1, clayr: 0, obsidr: 0, geor: 0 }];
    for _ in 0..n {
        let mut new: Vec<St> = v.iter().map(|x| tick(&bp, &x)).collect::<Vec<Vec<St>>>().concat();
        new.sort_by_key(|s| (s.geo, s.geor, s.obsidr, s.clayr, s.orer, s.obsid, s.clay, s.ore));
        new.reverse();
        new.truncate(100000);
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

    let mut s = new.clone();
    if s.buy_orer(st, &bp.orer, bp.max_orer()) {
        r.push(s);
    }

    let mut s = new.clone();
    if s.buy_clayr(st, &bp.clayr, bp.max_clayr()) {
        r.push(s);
    }

    let mut s = new.clone();
    if s.buy_obsidr(st, &bp.obsidr, bp.max_obsidr()) {
        r.push(s);
    }

    let mut s = new.clone();
    if s.buy_geodr(st, &bp.geor) {
        r.push(s);
    }

    r.push(new);

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

impl Blue {
    fn max_orer(&self) -> u32 {
        self.orer.ore.max(
            self.clayr.ore.max(
                self.obsidr.ore.max(
                    self.geor.ore
                    )
                )
            )
    }
    fn max_clayr(&self) -> u32 {
        self.obsidr.clay
    }
    fn max_obsidr(&self) -> u32 {
        self.geor.obsid
    }
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
    fn buy_orer(&mut self, cur: &St, c: &Cost, max: u32) -> bool {
        if cur.ore >= c.ore && self.orer < max {
            self.ore -= c.ore;
            self.orer += 1;
            true
        } else { false }
    }
    fn buy_clayr(&mut self, cur: &St, c: &Cost, max: u32) -> bool {
        if cur.ore >= c.ore && self.clayr < max {
            self.ore -= c.ore;
            self.clayr += 1;
            true
        } else { false }
    }
    fn buy_obsidr(&mut self, cur: &St, c: &Cost, max: u32) -> bool {
        if cur.ore >= c.ore && cur.clay >= c.clay && self.obsidr < max {
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

pub fn part_2(inp: &str) -> String {
    inp.lines()
        .map(parse_ln)
        .take(3)
        .map(|x| geos(32, x))
        .product::<u32>()
        .to_string()
}
