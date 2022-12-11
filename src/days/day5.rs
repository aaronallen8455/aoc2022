use std::collections::VecDeque;

pub fn part_1(inp: &str) -> String {
    let mut i = inp.split("\n\n");
    let h = i.next().unwrap();
    let v = parse_stacks(h);
    let num_stacks = (v[0].len() + 1) / 4;
    let mut stacks: Vec<VecDeque<char>> =
        (0..num_stacks).map(|x| get_stack(&v, x as usize)).collect();
    let is = parse_instrs(i.next().unwrap());
    for (a, b, c) in is {
        do_instr(&mut stacks, a, b, c)
    }
    stacks.iter().map(|q| q.front().unwrap()).collect()
}

fn parse_stacks(inp: &str) -> Vec<Vec<char>> {
    let mut v: Vec<Vec<char>> = inp.lines().map(|x| x.chars().collect()).collect();
    v.resize(v.len() - 1, Vec::new());
    v
}

fn get_stack(s: &Vec<Vec<char>>, i: usize) -> VecDeque<char> {
    let si = i * 4 + 1;
    let i = s.iter().map(|v| v[si]).filter(|c| *c != ' ');
    VecDeque::from_iter(i)
}

fn parse_instrs(i: &str) -> Vec<(usize, usize, usize)> {
    i.lines().map(parse_instr).collect()
}

fn parse_instr(l: &str) -> (usize, usize, usize) {
    let v: Vec<&str> = l.split(' ').collect();
    (v[1].parse().unwrap(), v[3].parse().unwrap(), v[5].parse().unwrap())
}

fn do_instr(stacks: &mut Vec<VecDeque<char>>, n: usize, from: usize, to: usize) {
    for _ in 0..n {
        let x = stacks[from - 1].pop_front().unwrap();
        stacks[to - 1].push_front(x);
    }
}

pub fn part_2(inp: &str) -> String {
    let mut i = inp.split("\n\n");
    let h = i.next().unwrap();
    let v = parse_stacks(h);
    let num_stacks = (v[0].len() + 1) / 4;
    let mut stacks: Vec<VecDeque<char>> =
        (0..num_stacks).map(|x| get_stack(&v, x as usize)).collect();
    let is = parse_instrs(i.next().unwrap());
    for (a, b, c) in is {
        do_instr2(&mut stacks, a, b, c)
    }
    stacks.iter().map(|q| q.front().unwrap()).collect()
}

fn do_instr2(stacks: &mut Vec<VecDeque<char>>, n: usize, from: usize, to: usize) {
    let mut crates = VecDeque::new();
    for _ in 0..n {
        crates.push_back(stacks[from - 1].pop_front().unwrap());
    }
    crates.append(&mut stacks[to - 1]);
    stacks[to - 1] = crates
}
