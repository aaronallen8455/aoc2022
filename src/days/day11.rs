use std::collections::VecDeque;

pub fn part_1(_inp: &str) -> String {
    let mut monks = vec![
        Monk {
            items: VecDeque::from([99, 63, 76, 93, 54, 73]),
            op: Box::new(|x| x * 11),
            test: Box::new(|x| if x % 2 == 0 { 7 } else { 1 }),
        },
        Monk {
            items: VecDeque::from([91, 60, 97, 54]),
            op: Box::new(|x| x + 1),
            test: Box::new(|x| if x % 17 == 0 { 3 } else { 2 }),
        },
        Monk {
            items: VecDeque::from([65]),
            op: Box::new(|x| x + 7),
            test: Box::new(|x| if x % 7 == 0 { 6 } else { 5 }),
        },
        Monk {
            items: VecDeque::from([84, 55]),
            op: Box::new(|x| x + 3),
            test: Box::new(|x| if x % 11 == 0 { 2 } else { 6 }),
        },
        Monk {
            items: VecDeque::from([86, 63, 79, 54, 83]),
            op: Box::new(|x| x * x),
            test: Box::new(|x| if x % 19 == 0 { 7 } else { 0 }),
        },
        Monk {
            items: VecDeque::from([96, 67, 56, 95, 64, 69, 96]),
            op: Box::new(|x| x + 4),
            test: Box::new(|x| if x % 5 == 0 { 4 } else { 0 }),
        },
        Monk {
            items: VecDeque::from([66, 94, 70, 93, 72, 67, 88, 51]),
            op: Box::new(|x| x * 5),
            test: Box::new(|x| if x % 13 == 0 { 4 } else { 5 }),
        },
        Monk {
            items: VecDeque::from([59, 59, 74]),
            op: Box::new(|x| x + 8),
            test: Box::new(|x| if x % 3 == 0 { 1 } else { 3 }),
        }
    ];
    let mut r = vec![0u64; 8];
    for _ in 0..20 {
        for i in 0..monks.len() {
            let monk = &mut monks[i];
            r[i] += monk.items.len() as u64;
            let mut thrown = Vec::new();
            while let Some(item) = monk.items.pop_front() {
                let new = chill((monk.op)(item));
                let ix = (monk.test)(new);
                thrown.push((ix, new));
            }
            for (ix, item) in thrown.iter() {
                monks[*ix].items.push_back(*item);
            }
        }
    }
    r.sort();
    (r[7] * r[6]).to_string()
}

fn chill(s: u64) -> u64 {
    s / 3
}

struct Monk where
{
    items: VecDeque<u64>,
    op: Box<dyn (Fn(u64) -> u64)>,
    test: Box<dyn (Fn(u64) -> usize)>,
}


pub fn part_2(_inp: &str) -> String {
    let mut monks = vec![
        Monk {
            items: VecDeque::from([99, 63, 76, 93, 54, 73]),
            op: Box::new(|x| x * 11),
            test: Box::new(|x| if x % 2 == 0 { 7 } else { 1 }),
        },
        Monk {
            items: VecDeque::from([91, 60, 97, 54]),
            op: Box::new(|x| x + 1),
            test: Box::new(|x| if x % 17 == 0 { 3 } else { 2 }),
        },
        Monk {
            items: VecDeque::from([65]),
            op: Box::new(|x| x + 7),
            test: Box::new(|x| if x % 7 == 0 { 6 } else { 5 }),
        },
        Monk {
            items: VecDeque::from([84, 55]),
            op: Box::new(|x| x + 3),
            test: Box::new(|x| if x % 11 == 0 { 2 } else { 6 }),
        },
        Monk {
            items: VecDeque::from([86, 63, 79, 54, 83]),
            op: Box::new(|x| x * x),
            test: Box::new(|x| if x % 19 == 0 { 7 } else { 0 }),
        },
        Monk {
            items: VecDeque::from([96, 67, 56, 95, 64, 69, 96]),
            op: Box::new(|x| x + 4),
            test: Box::new(|x| if x % 5 == 0 { 4 } else { 0 }),
        },
        Monk {
            items: VecDeque::from([66, 94, 70, 93, 72, 67, 88, 51]),
            op: Box::new(|x| x * 5),
            test: Box::new(|x| if x % 13 == 0 { 4 } else { 5 }),
        },
        Monk {
            items: VecDeque::from([59, 59, 74]),
            op: Box::new(|x| x + 8),
            test: Box::new(|x| if x % 3 == 0 { 1 } else { 3 }),
        }
    ];
    let mut r = vec![0u64; 8];
    for _ in 0..10000 {
        for i in 0..monks.len() {
            let monk = &mut monks[i];
            r[i] += monk.items.len() as u64;
            let mut thrown = Vec::new();
            while let Some(item) = monk.items.pop_front() {
                let new = chill2((monk.op)(item));
                let ix = (monk.test)(new);
                thrown.push((ix, new));
            }
            for (ix, item) in thrown.iter() {
                monks[*ix].items.push_back(*item);
            }
        }
    }
    r.sort();
    (r[7] * r[6]).to_string()
}

fn chill2(s: u64) -> u64 {
    s % (2 * 17 * 7 * 11 * 19 * 5 * 13 * 3)
}
