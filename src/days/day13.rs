use std::cmp::Ordering;
use std::iter::Peekable;
use std::collections::VecDeque;

pub fn part_1(inp: &str) -> String {
    let pairs = inp.split("\n\n");
    let mut res: Vec<usize> = Vec::new();
    for (i, pair) in pairs.enumerate() {
        let i = i + 1;
        match pair.lines().map(|x| parse_pack(&mut x.chars().peekable()))
            .collect::<Option<Vec<Pack>>>() {
            None => (),
            Some(v) => {
                let mut it = v.into_iter();
                let mut a = it.next().unwrap();
                let mut b = it.next().unwrap();
                if good(&mut a, &mut b) == Some(true) {
                    res.push(i);
                }
            }
        }
    }
    res.into_iter().sum::<usize>().to_string()
}

#[derive(Clone, PartialEq)]
enum Pack {
    L(VecDeque<Pack>),
    N(u32),
}

fn good(a: &mut Pack, b: &mut Pack) -> Option<bool> {
    match (a, b) {
        (Pack::N(a), Pack::N(b)) => {
            if a < b { return Some(true) }
            if b < a { return Some(false) }
            None
        },
        (Pack::N(a), b@Pack::L(_)) => good(&mut Pack::L(VecDeque::from([Pack::N(*a)])), b),
        (a@Pack::L(_), Pack::N(b)) => good(a, &mut Pack::L(VecDeque::from([Pack::N(*b)]))),
        (Pack::L(a), Pack::L(b)) => loop {
            match (a.pop_front(), b.pop_front()) {
                (None, Some(_)) => return Some(true),
                (Some(_), None) => return Some(false),
                (None, None) => return None,
                (Some(mut x), Some(mut y)) =>
                    match good(&mut x, &mut y) {
                        None => (),
                        Some(r) => return Some(r)
                    }
            }
        }
    }
}

fn parse_pack<I>(inp: &mut Peekable<I>) -> Option<Pack>
where I: Iterator<Item=char> {
    match inp.next()? {
        ']' => None,
        '[' => {
            let mut v = VecDeque::new();
            loop {
                if *inp.peek()? == ']' {
                    inp.next();
                    return Some(Pack::L(v))
                }
                if *inp.peek()? == ',' {
                    inp.next();
                }
                match parse_pack(inp) {
                    None => return Some(Pack::L(v)),
                    Some(pack) => {
                        v.push_back(pack);
                    }
                }
            }
        },
        x => {
            let mut num = vec![x];
            while inp.peek()?.is_digit(10) {
                num.push(inp.next().unwrap());
            }
            let num = num.iter().collect::<String>().parse::<u32>().ok()?;
            Some(Pack::N(num))
        }
    }
}

pub fn part_2(inp: &str) -> String {
    let pairs = inp.split("\n\n");
    let mut packs: Vec<Pack> = Vec::new();
    let x = pairs
        .map(|x| x.lines().map(|y| parse_pack(&mut y.chars().peekable()))
             .collect::<Option<Vec<Pack>>>())
        .collect::<Option<Vec<Vec<Pack>>>>();
    match x {
        None => panic!("ah shit"),
        Some(ps) => {
            for p in ps {
                for pp in p {
                    packs.push(pp);
                }
            }
        }
    };
    let pa = Pack::L(VecDeque::from([Pack::L(VecDeque::from([Pack::N(2)]))]));
    let pb = Pack::L(VecDeque::from([Pack::L(VecDeque::from([Pack::N(6)]))]));
    packs.push(pa.clone());
    packs.push(pb.clone());
    packs.sort_by(comp);
    packs.iter().enumerate().filter_map(|(i, x)| {
        if *x == pa || *x == pb { Some(i + 1) } else { None }
    }).product::<usize>().to_string()
}

fn comp(a: &Pack, b: &Pack) -> Ordering {
    match good(&mut a.clone(), &mut b.clone()) {
        Some(true) => Ordering::Less,
        Some(false) => Ordering::Greater,
        None => Ordering::Equal,
    }
}
