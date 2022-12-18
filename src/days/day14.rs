use std::collections::HashMap;
use itertools::Itertools;

pub fn part_1(inp: &str) -> String {
    let mut m = HashMap::new();
    let mut d = 0u32;
    for l in inp.lines() {
        do_line(&mut m, &mut d, l)
    }
    let mut count = 0;
    while let Ok(_) = drop(&mut m, d, false, (500,0)) { count += 1 }
    count.to_string()
}

type C = (u32, u32);
#[derive(Debug)]
enum Cell {
    S,
    R,
}

type Map = HashMap<C, Cell>;

fn do_line(m: &mut Map, depth: &mut u32, l: &str) {
    let coords = l.split(" -> ").map(|x| {
        let mut i = x.split(',');
        (i.next().unwrap().parse::<u32>().unwrap(), i.next().unwrap().parse::<u32>().unwrap())
    });
    for ((ax, ay), (bx, by)) in coords.tuple_windows() {
        *depth = (*depth).max(ay).max(by);
        for x in ax.min(bx) ..= ax.max(bx) {
            for y in ay.min(by) ..= ay.max(by) {
                m.insert((x, y), Cell::R);
            }
        }
    }
}

fn drop(m: &mut Map, depth: u32, is_floor: bool, target@(x, y): C) -> Result<bool, ()> {
    if !is_floor && y > depth { return Err(()); }
    if is_floor && y == depth { return Ok(false) }
    match m.get(&target) {
        None => Ok(
            drop(m, depth, is_floor, (x, y+1))?
                || {
                    m.insert(target, Cell::S);
                    true
                }),
        Some(Cell::S) if y == 0 => {
            Err(())
        }
        _ => Ok(
            (!m.contains_key(&(x-1, y)) && drop(m, depth, is_floor, (x-1, y))?)
                ||
            (!m.contains_key(&(x+1, y)) && drop(m, depth, is_floor, (x+1, y))?))
    }
}

pub fn part_2(inp: &str) -> String {
    let mut m = HashMap::new();
    let mut d = 0u32;
    for l in inp.lines() {
        do_line(&mut m, &mut d, l)
    }
    let mut count = 0;
    while let Ok(_) = drop(&mut m, d + 2, true, (500,0)) { count += 1 }
    count.to_string()
}
