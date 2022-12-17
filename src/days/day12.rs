use std::collections::HashSet;

pub fn part_1(inp: &str) -> String {
    let m: Vec<Vec<char>> = inp.lines()
        .map(|x| x.chars().collect())
        .collect();
    let start = find_start(&m);
    let res = search(&m, &mut HashSet::from([(start)]), 0, vec![start]);
    res.to_string()
}

type C = (usize, usize);

fn find_start(m: &Vec<Vec<char>>) -> C {
    for (ri, row) in m.iter().enumerate() {
        for (ci, col) in row.iter().enumerate() {
            if *col == 'S' { return (ri, ci) }
        }
    }
    (0,0)
}

fn search(m: &Vec<Vec<char>>, visited: &mut HashSet<C>, mut i: u32, mut fringe: Vec<C>) -> u32 {
    loop {
        let new_fringe =
            fringe.into_iter()
            .map(|x| paths(&m, visited, i, x))
            .collect::<Result<Vec<Vec<C>>, u32>>();
        match new_fringe {
            Err(r) => return r,
            Ok(new_fringe) => {
                let fuck = new_fringe.concat();
                fringe = fuck
            } // search(m, visited, i + 1, new_fringe.concat())
        }
        i += 1;
    }
}

fn paths(m: &Vec<Vec<char>>, visited: &mut HashSet<C>, i: u32, (r, c): C) -> Result<Vec<C>, u32> {
    let cur = m[r][c];
    if cur == 'E' {
        return Err(i)
    }
    let r = r as isize;
    let c = c as isize;
    Ok(
        [(r, c-1), (r, c+1), (r-1, c), (r+1, c)]
            .into_iter()
            .filter_map(|(rr, cc)| {
                let rr = usize::try_from(rr).ok()?;
                let cc = usize::try_from(cc).ok()?;
                let ch = get(&m, (rr, cc))?;
                if !visited.contains(&(rr, cc)) && travel(&cur, &ch) {
                    visited.insert((rr, cc));
                    Some((rr, cc))
                } else {
                    None
                }
            }).collect()
    )
}

fn get(m: &Vec<Vec<char>>, (r, c): C) -> Option<char> {
    let m = m.get(r)?;
    let &y = m.get(c)?;
    Some(y)
}

fn travel(a: &char, b: &char) -> bool {
    let a = if *a == 'S' { &'a' } else { a };
    let b = if *b == 'E' { &'z' } else { b };
    if b <= a { return true }
    *b as i8 - *a as i8 == 1
}

pub fn part_2(inp: &str) -> String {
    let m: Vec<Vec<char>> = inp.lines()
        .map(|x| x.chars().collect())
        .collect();
    let m = invert(m);
    let start = find_start(&m);
    let res = search(&m, &mut HashSet::from([(start)]), 0, vec![start]);
    res.to_string()
}

fn invert(m: Vec<Vec<char>>) -> Vec<Vec<char>> {
    m.into_iter()
        .map(|v| v.into_iter().map(|x| {
            if x == 'E' { return 'S' }
            if x == 'S' || x == 'a' { return 'E' }
            ('z' as i8 - x as i8 + 'a' as i8) as u8 as char
        }).collect())
        .collect()
}
