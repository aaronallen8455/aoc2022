pub fn part_1(inp: &str) -> String {
    let mut v: Vec<(i64, bool)> = inp.lines().map(|l| (l.parse::<i64>().unwrap(), false)).collect();
    let mut i = 0usize;
    while i < v.len() {
        let (x, b) = v[i];
        if b {
            i += 1;
            continue;
        }
        v.remove(i);
        let new_i = ((i as i64 + x).rem_euclid(v.len() as i64)) as usize;
        let new_i = if x < 0 && new_i == 0 { v.len() } else { new_i };
        v.insert(new_i, (x, true));
        if new_i <= i {
            i += 1;
        }
    }
    let zi = find_zero(&v);
    let a = v[(zi + 1000) % v.len()];
    let b = v[(zi + 2000) % v.len()];
    let c = v[(zi + 3000) % v.len()];
    (a.0 + b.0 + c.0).to_string()
}

fn find_zero(v: &Vec<(i64, bool)>) -> usize {
    for i in 0..v.len() {
        if v[i].0 == 0 { return i }
    }
    0
}

fn find_zero2(v: &Vec<i64>) -> usize {
    for i in 0..v.len() {
        if v[i] == 0 { return i }
    }
    0
}

pub fn part_2(inp: &str) -> String {
    // would be faster to have a vector of each value along with its index then
    // do a scan over that producing the new indexes along with a function that
    // modifies an index built up from all transformations. Would then be O(n)
    // for each remix.
    let mut v: Vec<i64> = inp.lines().map(|l| l.parse::<i64>().unwrap() * 811589153).collect();
    let mut vi = Vec::from_iter(0..v.len());
    for _ in 0..10 {
        for ii in 0..vi.len() {
            let i = vi[ii];
            let x = v[i];
            v.remove(i);
            let new_i = ((i as i64 + x).rem_euclid(v.len() as i64)) as usize;
            let new_i = if x < 0 && new_i == 0 { v.len() } else { new_i };
            v.insert(new_i, x);
            for iii in 0..vi.len() {
                if vi[iii] > i {
                    vi[iii] -= 1;
                }
                if vi[iii] >= new_i {
                    vi[iii] += 1;
                }
            }

            vi[ii] = new_i;
        }
    }
    let zi = find_zero2(&v);
    let a = v[(zi + 1000) % v.len()];
    let b = v[(zi + 2000) % v.len()];
    let c = v[(zi + 3000) % v.len()];
    (a + b + c).to_string()
}
