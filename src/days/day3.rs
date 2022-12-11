use std::collections::{HashSet, HashMap};
use itertools::Itertools;

pub fn part_1(inp: &str) -> String {
    inp.lines()
        .map(do_it)
        .map(convert)
        .sum::<u32>()
        .to_string()
}

fn do_it(inp: &str) -> char {
    let (r1, r2) = inp.split_at(inp.len() / 2);
    let f: HashSet<char> = HashSet::from_iter(r1.chars());
    let y: HashSet<char> = HashSet::from_iter(r2.chars());
    let c: HashSet<&char> = f.intersection(&y).collect();
    *c.into_iter().next().unwrap()
}

fn convert(i: char) -> u32 {
    let m: HashMap<char, usize> =
        HashMap::from_iter(('a'..='z').chain('A'..='Z').enumerate().map(swap));
    *m.get(&i).unwrap() as u32 + 1
}

fn swap<A,B>(i: (A, B)) -> (B, A) {
    (i.1, i.0)
}

pub fn part_2(inp: &str) -> String {
    inp.lines()
        .chunks(3)
        .into_iter()
        .map(solve)
        .map(convert)
        .sum::<u32>()
        .to_string()
}

fn solve<'a, I>(inp: I) -> char
where I: Iterator<Item = &'a str> {
    let mut iter = inp.map(|x| HashSet::from_iter(x.chars()));
    let a: HashSet<char> = iter.next().unwrap();
    let b = iter.next().unwrap();
    let c = iter.next().unwrap();
    let i: HashSet<char> = a.intersection(&b).map(|x| *x).collect();
    let i: HashSet<&char> = i.intersection(&c).collect();
    *i.into_iter().next().unwrap()
}
