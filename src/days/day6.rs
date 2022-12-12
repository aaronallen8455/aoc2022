use std::str;
use itertools::Itertools;
use std::collections::HashSet;

pub fn part_1(inp: &str) -> String {
    (inp.chars()
        .tuple_windows::<(_,_,_,_)>()
        .position(the_one)
        .unwrap() + 4)
        .to_string()
}

fn the_one(i: (char, char, char, char)) -> bool {
    let (a,b,c,d) = i;
    a != b && a != c && a != d
        && b != c && b != d
        && c != d
}

pub fn part_2(inp: &str) -> String {
    (inp.as_bytes().windows(14).position(the_one2).unwrap() + 14).to_string()
}

fn the_one2(i: &[u8]) -> bool {
    let h: HashSet<&u8> = HashSet::from_iter(i);
    h.len() == 14
}
