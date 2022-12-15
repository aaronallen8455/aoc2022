use itertools::Itertools;

pub fn part_1(inp: &str) -> String {
    let mut v = Vec::new();
    inp.lines()
        .map(parse_instr)
        .fold(1, |nxt, instr| {
            match instr {
                Instr::Add(n) => {
                    v.push(nxt);
                    v.push(nxt);
                    n + nxt
                },
                Instr::Noop => {
                    v.push(nxt);
                    nxt
                }
            }
        });
    (v[19] * 20 + v[59] * 60 + v[99] * 100 + v[139] * 140 + v[179] * 180 + v[219] * 220)
        .to_string()
}

enum Instr {
    Add(i32),
    Noop,
}

fn parse_instr(inp: &str) -> Instr {
    match inp.split_at(4) {
        ("addx", n) => Instr::Add(n.trim().parse().unwrap()),
        ("noop", _) => Instr::Noop,
        _ => panic!("panic!")
    }
}

pub fn part_2(inp: &str) -> String {
    let mut v = Vec::new();
    inp.lines()
        .map(parse_instr)
        .fold(1, |nxt, instr| {
            match instr {
                Instr::Add(n) => {
                    v.push(nxt);
                    v.push(nxt);
                    n + nxt
                },
                Instr::Noop => {
                    v.push(nxt);
                    nxt
                }
            }
        });
    v.iter().enumerate().map(|(i, x)| {
        if (i % 40) as i32 >= x - 1 && (i % 40) as i32 <= x + 1 {
            'X'
        } else {
            '.'
        }
    }).chunks(40).into_iter().take(6).map(|x| x.collect::<String>()).join("\n")
}
