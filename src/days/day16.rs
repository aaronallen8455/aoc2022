use std::collections::{HashMap, HashSet};

pub fn part_1(inp: &str) -> String {
    let graph: Graph = inp.lines().map(parse_ln).collect();
    let ds = dists(&graph);
    let non_zs: Vec<&String> = graph
        .iter()
        .filter(|(_, v)| v.rate > 0)
        .map(|x| x.0)
        .collect();
    let mut r = 0u32;
    search(&mut r, &graph, &non_zs, ds, vec![St { visited: HashSet::new(), loc: "AA".to_string(), time_left: 30, total: 0 }]);
    r.to_string()
}

fn search(res: &mut u32, graph: &Graph, ns: &Vec<&String>, dists: HashMap<(&String, &String), u32>, cands: Vec<St>) {
    *res = *res.max(&mut cands.iter().map(|x| x.total).max().unwrap());
    let new_c = cands.iter()
        .map(|c| new_cands(&graph, ns, &dists, c))
        .flatten()
        .collect::<Vec<St>>();
    if new_c.len() > 0 {
        search(res, graph, ns, dists, new_c)
    }
}

fn new_cands(graph: &Graph, ns: &Vec<&String>, dists: &HashMap<(&String, &String), u32>, cand: &St) -> Vec<St> {
    let mut r = Vec::new();
    for n in ns.iter().filter(|x| !cand.visited.contains(**x)) {
        let d = dists.get(&(&cand.loc, n)).unwrap();
        let node = graph.get(*n).unwrap();
        let mut visited = cand.visited.clone();
        visited.insert((*n).clone());
        let loc = n;
        let time_left = cand.time_left - *d as i32 - 1;
        if time_left > 0 {
            let total = cand.total + time_left as u32 * node.rate;
            let new_c = St {
                visited,
                loc: (*loc).clone(),
                time_left,
                total
            };
            r.push(new_c)
        }
    }
    r
}

#[derive(Debug)]
struct Node {
    rate: u32,
    tunnels: Vec<String>,
}

#[derive(Clone)]
struct St {
    visited: HashSet<String>,
    loc: String,
    time_left: i32,
    total: u32
}

type Graph = HashMap<String, Node>;

fn parse_ln(l: &str) -> (String, Node) {
    let mut i = l.split_whitespace();
    i.next();
    let name = i.next().unwrap();
    i.next();
    i.next();
    let rate = i.next().unwrap().strip_prefix("rate=").unwrap()
        .strip_suffix(";").unwrap().parse::<u32>().unwrap();
    i.next();
    i.next();
    i.next();
    i.next();
    let tunnels = i.map(|x| x.strip_suffix(",").unwrap_or(x).to_string()).collect();
    (name.to_string(), Node {rate, tunnels})
}

fn dists(graph: &Graph) -> HashMap<(&String, &String), u32> {
    let mut res: HashMap<(&String, &String), u32> = HashMap::new();
    for (k, v) in graph.iter() {
        for t in v.tunnels.iter() {
            res.insert((k, t), 1);
        }
    }
    let mut keys: Vec<&String> = graph.keys().collect();
    keys.sort();
    for i in 0..keys.len() {
        for s in 0..keys.len() {
            for e in 0..keys.len() {
                let mut ad = None;
                if let Some(d1) = res.get(&(keys[s], keys[i])) {
                    if let Some(d2) = res.get(&(keys[i], keys[e])) {
                        ad = Some(*d1 + *d2);
                    }
                }
                match ad {
                    Some(mut added) => {
                        res.entry((keys[s], keys[e]))
                            .and_modify(|d| *d = *d.min(&mut added))
                            .or_insert(added);},
                    None => ()
                }
            }
        }
    }
    res
}

pub fn part_2(inp: &str) -> String {
    let graph: Graph = inp.lines().map(parse_ln).collect();
    let ds = dists(&graph);
    let non_zs: Vec<&String> = graph
        .iter()
        .filter(|(_, v)| v.rate > 0)
        .map(|x| x.0)
        .collect();
    let mut r = 0u32;
    search2(&mut r, &graph, &non_zs, ds,
            vec![(St { visited: HashSet::new(), loc: "AA".to_string(), time_left: 26, total: 0 }
             , St { visited: HashSet::new(), loc: "AA".to_string(), time_left: 26, total: 0 })]
             );
    r.to_string()
}

fn search2(res: &mut u32,
           graph: &Graph,
           ns: &Vec<&String>,
           dists: HashMap<(&String, &String), u32>,
           cands: Vec<(St, St)>) {
    *res = *res.max(&mut cands.iter().map(|(x, y)| x.total + y.total).max().unwrap());
    let mut new_c = cands.iter()
        .map(|c| new_cands2(&graph, ns, &dists, c))
        .flatten()
        .collect::<Vec<(St, St)>>();
    if new_c.len() > 0 {
        new_c.sort_by_key(|(a1, b1)| a1.total + b1.total);
        new_c = new_c.into_iter().rev().take(4000).collect();
        search2(res, graph, ns, dists, new_c)
    }
}

fn new_cands2(graph: &Graph,
              ns: &Vec<&String>,
              dists: &HashMap<(&String, &String), u32>,
              (cand1, cand2): &(St, St))
                                -> Vec<(St, St)> {
    let mut r = Vec::new();
    let ns: Vec<&String> = ns.iter().filter(|x| !cand1.visited.contains(**x) && !cand2.visited.contains(**x)).map(|x| *x).collect();
    if ns.len() == 1 {
        r.push((cand1.clone(), upd(graph, dists, &cand2, ns[0])));
    } else {
        for n1 in ns.iter() {
            for n2 in ns.iter() {
                if n1 == n2 { continue }
                let le = upd(graph, dists, &cand1, n1);
                let ri = upd(graph, dists, &cand2, n2);
                r.push((le, ri));
            }
        }
    }
    // filter out where no remaining time for both
    r.into_iter().filter(|(c1, c2)| c1.time_left + c2.time_left > 0).collect()
}

fn upd(graph: &Graph, dists: &HashMap<(&String, &String), u32>, cand: &St, n: &String) -> St {
    let d = dists.get(&(&cand.loc, n)).unwrap();
    let node = graph.get(n).unwrap();
    let mut visited = cand.visited.clone();
    let loc = n;
    let time_left = 0.max(cand.time_left - *d as i32 - 1);
    if time_left > 0 {
        visited.insert((*n).clone());
    }
    let total = cand.total + time_left as u32 * node.rate;
    let new_c = St {
        visited,
        loc: (*loc).clone(),
        time_left,
        total
    };
    new_c
}
