use std::collections::HashSet;

pub fn part_1(inp: &str) -> String {
    let forest: Vec<Vec<i32>> = inp.lines()
        .map(|s| s.chars().filter_map(|x| x.to_digit(10)).map(|x| x as i32).collect())
        .collect();
    let a = yup(&forest);
    let b = yup(&transpose(&forest));
    let b = b.iter().map(|(x,y)| (*y,*x)).collect();
    let c: HashSet<(usize, usize)> = a.union(&b).map(|x| *x).collect();
    c.len().to_string()
}

pub fn part_2(inp: &str) -> String {
    let forest: Vec<Vec<i32>> = inp.lines()
        .map(|s| s.chars().filter_map(|x| x.to_digit(10)).map(|x| x as i32).collect())
        .collect();
    let mut r = 0;
    for i in 0..forest.len() {
        for j in 0..forest.len() {
            let x = count_em(i, j, &forest);
            r = r.max(x);
        }
    }
    r.to_string()
}

fn count_em(r: usize, c: usize, f: &Vec<Vec<i32>>) -> i32 {
    let t = f[r][c];
    let mut acc = 0;
    let mut res = 0;
    if r != 0 {
        for i in (0..=r-1).rev() {
            res += 1;
            if f[i][c] >= t {
                break;
            }
        }
    }
    for i in r+1..f.len() {
        acc += 1;
        if f[i][c] >= t {
            break;
        }
    }
    res *= acc;
    acc = 0;
    if c != 0 {
        for i in (0..=c-1).rev() {
            acc += 1;
            if f[r][i] >= t {
                break;
            }
        }
    }
    res *= acc;
    acc = 0;
    for i in c+1..f.len() {
        acc += 1;
        if f[r][i] >= t {
            break;
        }
    }
    res *= acc;
    res
}

fn transpose(v: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut r = Vec::new();
    for i in 0..v.len() {
        let mut t: Vec<i32> = Vec::new();
        for j in 0..v.len() {
            t.push(v[j][i]);
        }
        r.push(t);
    }
    r
}

fn yup(v: &Vec<Vec<i32>>) -> HashSet<(usize, usize)> {
    v.iter().enumerate()
        .map(|(ix, x)| {
            let h = do_it(x);
            h.iter().map(|o| (ix, *o)).collect::<HashSet<(usize, usize)>>()
        }).fold(HashSet::new(), |acc, x| acc.union(&x).map(|x| *x).collect())
}

fn do_it(v: &Vec<i32>) -> HashSet<usize> {
    let a = visible(v);
    let b = visible(&v.iter().map(|x| *x).rev().collect());
    let b = b.iter().map(|x| v.len() - 1 - x).collect();
    a.union(&b).map(|x| *x).collect()
}

fn visible(v: &Vec<i32>) -> HashSet<usize> {
    v.iter().enumerate().fold((HashSet::new(), -1), |(mut r, acc), (ix, &x)| {
        if x > acc {
            r.insert(ix);
            (r, x)
        } else {
            (r, acc)
        }
    }).0
}
