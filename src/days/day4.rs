pub fn part_1(inp: &str) -> String {
    inp.lines()
        .map(do_it)
        .sum::<u32>()
        .to_string()
}

fn do_it(inp: &str) -> u32 {
    inp.lines()
        .map(parse_line)
        .filter(contained)
        .count() as u32
}

fn parse_line(inp: &str) -> ((u32, u32), (u32, u32)) {
    let mut i = inp.split(',').map(parse_range);
    (i.next().unwrap(), i.next().unwrap())
}

fn parse_range(inp: &str) -> (u32, u32) {
    let mut i = inp.split('-').map(|x| x.parse::<u32>().unwrap());
    (i.next().unwrap(), i.next().unwrap())
}

fn contained(i: &((u32, u32), (u32, u32))) -> bool {
    (i.0.0 >= i.1.0 && i.0.1 <= i.1.1)
        ||
        (i.1.0 >= i.0.0 && i.1.1 <= i.0.1)
}

pub fn part_2(inp: &str) -> String {
    inp.lines()
        .map(do_it_again)
        .sum::<u32>()
        .to_string()
}

fn do_it_again(inp: &str) -> u32 {
    inp.lines()
        .map(parse_line)
        .filter(overlap)
        .count() as u32
}

fn overlap(i: &((u32, u32), (u32, u32))) -> bool {
    (i.0.0 <= i.1.0 && i.0.1 >= i.1.0) ||
        (i.1.0 <= i.0.0 && i.1.1 >= i.0.0)
}
