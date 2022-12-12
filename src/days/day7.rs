use std::collections::HashMap;

pub fn part_1(inp: &str) -> String {
    let mut node = Node { content: HashMap::new(), size: None, is_top: true };
    let mut cmds = inp.lines()
        .map(parse_line);
    do_stuff(&mut node, &mut cmds);
    small_dirs(&mut node).to_string()
}

fn do_stuff<I>(n: &mut Node, iter: &mut I) -> Option<Where> where
I: Iterator<Item = Line> {
    let l = iter.next()?;
    match l {
        Line::Cd(CdT::Name(dir)) => {
            let d = n.content.get_mut(&dir).unwrap();
            match do_stuff(d, iter)? {
                Where::Back => do_stuff(n, iter),
                Where::Top => Some(Where::Top)
            }
        },
        Line::Cd(CdT::Back) => Some(Where::Back),
        Line::Cd(CdT::Root) => {
            if n.is_top {
                do_stuff(n, iter)
            } else { Some(Where::Top) }
        },
        Line::Ls => do_stuff(n, iter),
        Line::Dir(dir) => {
            let new = Node { content: HashMap::new(), size: None, is_top: false };
            n.content.insert(dir, new);
            do_stuff(n, iter)
        },
        Line::File(sz, nm) => {
            let new = Node { content: HashMap::new(), size: Some(sz), is_top: false };
            n.content.insert(nm, new);
            do_stuff(n, iter)
        }
    }
}

fn small_dirs(n: &mut Node) -> u32 {
    match n.size {
        None => {
            let result = n.content.values_mut().map(small_dirs).sum();
            let acc = n.content.values().filter_map(|x| x.size).sum();
            n.size = Some(acc);
            if acc <= 100000 {
                result + acc
            } else {
                result
            }
        },
        Some(_) => {
            0
        }
    }
}

enum Where {
    Back,
    Top,
}

pub fn part_2(inp: &str) -> String {
    let mut node = Node { content: HashMap::new(), size: None, is_top: true };
    let mut cmds = inp.lines()
        .map(parse_line);
    do_stuff(&mut node, &mut cmds);
    small_dirs(&mut node);
    let total = node.size.unwrap();
    let free = 30000000 - (70000000 - total);
    find_it(&node, free).unwrap().to_string()
}

fn find_it(n: &Node, need: u32) -> Option<u32> {
    match n.size {
        Some(x) if x >= need => {
            match n.content.values().filter_map(|x| find_it(x, need)).min() {
                None => n.size,
                Some(r) => Some(r),
            }
        },
        _ => None
    }
}

struct Node {
    content: HashMap<String, Node>,
    size: Option<u32>,
    is_top: bool,
}

enum Line {
    Cd(CdT),
    Ls,
    Dir(String),
    File(u32, String),
}

enum CdT {
    Name(String),
    Root,
    Back,
}

fn parse_line(i: &str) -> Line {
    if let Some(x) = i.strip_prefix("$ cd ") {
        match x {
            "/" => Line::Cd(CdT::Root),
            ".." => Line::Cd(CdT::Back),
            _ => Line::Cd(CdT::Name(x.to_string()))
        }
    } else if let Some("") = i.strip_prefix("$ ls") {
        Line::Ls
    } else if let Some(dir_name) = i.strip_prefix("dir ") {
        Line::Dir(dir_name.to_string())
    } else {
        let mut w = i.split_whitespace();
        Line::File(w.next().unwrap().parse().unwrap(), w.next().unwrap().to_string())
    }
}
