use std::collections::HashSet;

pub fn part_1(inp: &str) -> String {
    let mut itr = inp.chars().filter(|x| x != &'\n').cycle().map(parse_ln);
    let mut shpi = (0..=4).cycle();
    let mut top = 0i64;
    let mut nrocks = 0u32;
    let mut map = HashSet::new();
    while nrocks < 2022 {
        let shape = shpi.next().unwrap();
        let mut r = spawn(shape, top);
        loop {
            let (vt, hz) = itr.next().unwrap();
            mv(&mut map, &mut top, &mut r, hz, vt);
            if mv(&mut map, &mut top, &mut r, 0, -1) {
                    nrocks += 1;
                    break;
                }
        }
    }
    top.to_string()
}

fn parse_ln(i: char) -> (i64, i8) {
    match i {
        '>' => (0, 1),
        '<' => (0, -1),
        _ => (0, 0),
    }
}

type C = (i64, i8);
type Map = HashSet<C>;

fn spawn(shape: u8, top: i64) -> Vec<C> {
    let mut v = Vec::new();
    match shape {
        0 => {
            v.push((top + 4, 2));
            v.push((top + 4, 3));
            v.push((top + 4, 4));
            v.push((top + 4, 5));
        },
        1 => {
            v.push((top + 6, 3));
            v.push((top + 5, 3));
            v.push((top + 4, 3));
            v.push((top + 5, 2));
            v.push((top + 5, 4));
        },
        2 => {
            v.push((top + 4, 2));
            v.push((top + 4, 3));
            v.push((top + 4, 4));
            v.push((top + 5, 4));
            v.push((top + 6, 4));
        },
        3 => {
            v.push((top + 4, 2));
            v.push((top + 5, 2));
            v.push((top + 6, 2));
            v.push((top + 7, 2));
        },
        4 => {
            v.push((top + 4, 2));
            v.push((top + 5, 2));
            v.push((top + 4, 3));
            v.push((top + 5, 3));
        },
        _ => ()
    }
    v
}

fn mv(map: &mut Map, top: &mut i64, rock: &mut Vec<C>, hz: i8, vt: i64) -> bool {
    let mut new_rock = Vec::new();
    for (r, c) in rock.iter() {
        let new_r = r + vt;
        let new_c = c + hz;
        if map.contains(&(new_r, new_c)) || new_c < 0 || new_c > 6 || new_r == 0 {
            new_rock.push(None);
        } else {
            new_rock.push(Some((new_r, new_c)));
        }
    }
    match new_rock.into_iter().collect::<Option<Vec<C>>>() {
        None if vt != 0 => {
            for (r, c) in rock.into_iter() {
                map.insert((*r, *c));
                *top = *top.max(r);
            }
            true
        },
        None => false,
        Some(nr) => {
            *rock = nr;
            false
        }
    }
}

pub fn part_2(inp: &str) -> String {
    let mut itr = inp.chars().filter(|x| x != &'\n').cycle().map(parse_ln);
    let mut shpi = (0..=4).cycle();
    let mut top = 0i64;
    let mut nrocks = 0u32;
    let mut map = HashSet::new();
    let mut pile = Vec::new();
    while nrocks < 1600 + 1725 {
        let shape = shpi.next().unwrap();
        let mut r = spawn(shape, top);
        loop {
            let (vt, hz) = itr.next().unwrap();
            mv(&mut map, &mut top, &mut r, hz, vt);
            if mv(&mut map, &mut top, &mut r, 0, -1) {
                    nrocks += 1;
                    break;
                }
        }
        pile.push(top);
    }
    // 1000000000000 `divMod` 1725 = (579710144,1600)
    top += 579710143 * 2709;
    //let x: Vec<i64> = pile.windows(2).rev().map(|a| a[0] - a[1]).collect();
    //println!("{:?}", find_loop(x));
    top.to_string()
}

// 1725
// fn find_loop(v: Vec<i64>) -> usize {
//     for i in 1..v.len() {
//         let mut b = true;
//         for ii in 0..i {
//             if v[ii] != v[i + ii] || v[i + ii] != v[i * 2 + ii] || v[i * 2 + ii] != v[i * 3 + ii]{
//                 b = false;
//                 break;
//             }
//         }
//         if b { return i }
//     }
//     0
// }
