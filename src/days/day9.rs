use std::collections::HashSet;

pub fn part_1(inp: &str) -> String {
    let mut r = HashSet::from([(0, 0)]);
    inp.lines()
        .fold(((0, 0), (0, 0)), |(h, t), i| {
            let mvs = mv_head(h, i);
            let &the_new_h = mvs.last().unwrap();
            let new_t = mvs.into_iter().fold(t, |acc, x| {
                let n = mv_tail(x, acc);
                r.insert(n);
                n
            });
            (the_new_h, new_t)
        });
    r.len().to_string()
}

pub fn part_2(inp: &str) -> String {
    let mut r = HashSet::from([(0, 0)]);
    inp.lines()
        .fold(((0,0), vec![(0,0); 9]), |(h, knots), i| {
            let mvs = mv_head(h, i);
            let &the_next_h = mvs.last().unwrap();
            let new_knots = mvs.into_iter().fold(knots, |knots, new_h| {
                let new_knots = knots.into_iter().scan(new_h, |hh, k| {
                    let new_t = mv_tail(*hh, k);
                    *hh = new_t;
                    Some(new_t)
                });
                let new_knots: Vec<C> = new_knots.collect();
                let &the_one = new_knots.last().unwrap();
                r.insert(the_one);
                new_knots
            });
            (the_next_h, new_knots)
        });
    r.len().to_string()
}

type C = (i32, i32);

fn mv_tail((hr, hc): C, (tr, tc): C) -> C {
    let cd = hc - tc;
    let rd = hr - tr;
    if cd.abs() <= 1 && rd.abs() <= 1 {
        return (tr, tc);
    }
    let c_step = if cd < 0 { -1 } else if cd == 0 { 0 } else { 1 };
    let r_step = if rd < 0 { -1 } else if rd == 0 { 0 } else { 1 };
    (tr + r_step, tc + c_step)
}

fn mv_head((hr, hc): C, inp: &str) -> Vec<C> {
    let (n, r_step, c_step) = match inp.split_at(2) {
        ("U ", n) => (n.parse::<i32>().unwrap(), 1, 0),
        ("D ", n) => (n.parse::<i32>().unwrap(), -1, 0),
        ("L ", n) => (n.parse::<i32>().unwrap(), 0, -1),
        ("R ", n) => (n.parse::<i32>().unwrap(), 0, 1),
        _ => {
            println!("No");
            (0, 0, 0)
        }
    };
    (0..n).scan((hr, hc), |s, _| {
        s.0 += r_step;
        s.1 += c_step;
        Some((s.0, s.1))
    }).collect()
}
