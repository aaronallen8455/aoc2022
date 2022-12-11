pub fn part_1(inp: &str) -> String {
    inp.lines()
        .map(determine_score)
        .sum::<u32>()
        .to_string()
}

fn determine_score(inp: &str) -> u32 {
    let mut iter = inp.split(' ');
    let p1 = iter.next().unwrap();
    let p2 = iter.next().unwrap();
    let base = match p2 {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => 0,
    };
    base + beats(p1, p2)
}

fn beats(a: &str, b: &str) -> u32 {
    match a {
        "A" => match b {
            "X" => 3,
            "Y" => 6,
            "Z" => 0,
            _ => 0,
        },
        "B" => match b {
            "X" => 0,
            "Y" => 3,
            "Z" => 6,
            _ => 0,
        },
        "C" => match b {
            "X" => 6,
            "Y" => 0,
            "Z" => 3,
            _ => 0,
        },
        _ => 0,
    }
}

pub fn part_2(inp: &str) -> String {
    inp.lines()
        .map(do_it)
        .sum::<u32>()
        .to_string()
}

fn do_it(inp: &str) -> u32 {
    let mut iter = inp.split(' ');
    let p1 = iter.next().unwrap();
    let p2 = iter.next().unwrap();
    let c = match p2 {
        "X" => pick_lose(p1),
        "Y" => pick_draw(p1),
        "Z" => pick_win(p1),
        _ => "F"
    };
    let base = match c {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => 0,
    };
    base + beats(p1, c)
}

fn pick_win(inp: &str) -> &str {
    match inp {
        "A" => "Y",
        "B" => "Z",
        "C" => "X",
        _ => "F",
    }
}

fn pick_lose(inp: &str) -> &str {
    match inp {
        "A" => "Z",
        "B" => "X",
        "C" => "Y",
        _ => "F"
    }
}

fn pick_draw(inp: &str) -> &str {
    match inp {
        "A" => "X",
        "B" => "Y",
        "C" => "Z",
        _ => "F",
    }
}
