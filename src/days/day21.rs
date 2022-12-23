use std::collections::HashMap;

pub fn part_1(inp: &str) -> String {
    let mut m: HashMap<String, Monk> = HashMap::from_iter(inp.lines().map(parse_ln));
    while m.len() > 1 {
        reduce(&mut m);
    }
    match m.get(&"root".to_string()) {
        Some(Monk::Leaf(n)) => n.to_string(),
        _ => "failure".to_string(),
    }
}

fn reduce(m: &mut HashMap<String, Monk>) {
    let (leaf, mut bran): (HashMap<String, Monk>, HashMap<String, Monk>) =
                       m.into_iter().map(|(k, v)| (k.clone(), v.clone())).partition(|(_, m)| {
        match m {
            Monk::Leaf(_) => true,
            _ => false,
        }
    });
    for x in bran.values_mut() {
        match x {
            Monk::Branch(a, b, op) => {
                match (sub(a, &leaf), sub(b, &leaf)) {
                    (Some(a), Some(b)) => *x = Monk::Leaf(do_op(a, b, op)),
                    (Some(a), _) => *x = Monk::Branch(Opan::N(a), b.clone(), op.clone()),
                    (_, Some(b)) => *x = Monk::Branch(a.clone(), Opan::N(b), op.clone()),
                    _ => ()
                }
            },
            _ => ()
        }
    }
    *m = bran;
}

fn do_op(a: i64, b: i64, op: &Op) -> i64 {
    match op {
        Op::Add => a + b,
        Op::Sub => a - b,
        Op::Mul => a * b,
        Op::Div => a / b,
    }
}

fn sub(o: &Opan, m: &HashMap<String, Monk>) -> Option<i64> {
    match o {
        Opan::N(v) => Some(*v),
        Opan::V(v) =>
            match m.get(v) {
                Some(Monk::Leaf(n)) => Some(*n),
                _ => None
            }
    }
}

fn parse_ln (i: &str) -> (String, Monk) {
    parse_mop(i).or(parse_ml(i)).unwrap()
}

fn parse_mop (i: &str) -> Option<(String, Monk)> {
    let mut itr = i.split_whitespace();
    let name = itr.next()?.strip_suffix(":")?.to_string();
    let a = itr.next()?.to_string();
    let op = parse_op(itr.next()?);
    let b = itr.next()?.to_string();
    Some((name, Monk::Branch(Opan::V(a), Opan::V(b), op)))
}

fn parse_ml (i: &str) -> Option<(String, Monk)> {
    let mut itr = i.split_whitespace();
    let name = itr.next()?.strip_suffix(":")?.to_string();
    let n = itr.next()?.parse().ok()?;
    Some((name, Monk::Leaf(n)))
}

fn parse_op (i: &str) -> Op {
    match i {
        "/" => Op::Div,
        "*" => Op::Mul,
        "+" => Op::Add,
        "-" => Op::Sub,
        _ => panic!("fk"),
    }
}

#[derive(Clone)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Clone)]
enum Opan {
    N(i64),
    V(String),
}

#[derive(Clone)]
enum Monk {
    Leaf(i64),
    Branch(Opan, Opan, Op)
}

pub fn part_2(inp: &str) -> String {
    let m: HashMap<String, Monk> = HashMap::from_iter(inp.lines().map(parse_ln));
    match m.get("root") {
        Some(Monk::Branch(a, b, _)) => print_monk(&Monk::Branch(a.clone(), b.clone(), Op::Sub), &m),
        _ => "fail".to_string(),
    }
//    let mut r = 0;
//    for x in 3423279500000..=3423280000000 {
//        //println!("{}", monsta(x));
//        if monsta(x) == 0 {
//            r = x;
//            break;
//        }
//    }
//    r.to_string()
}

fn print_monk(m: &Monk, map: &HashMap<String, Monk>) -> String {
    match m {
        Monk::Leaf(n) => n.to_string(),
        Monk::Branch(a, b, op) =>
            format!("(({}) {} ({}))",
                print_opand(a, map),
                print_op(op),
                print_opand(b, map)),
    }
}

fn print_opand(o: &Opan, map: &HashMap<String, Monk>) -> String {
    match o {
        Opan::V(v) if v == &"humn".to_string() => "x".to_string(),
        Opan::V(v) => print_monk(map.get(v).unwrap(), map),
        Opan::N(n) => n.to_string(),
    }
}

fn print_op(op: &Op) -> String {
    match op {
        Op::Add => "+".to_string(),
        Op::Sub => "-".to_string(),
        Op::Mul => "*".to_string(),
        Op::Div => "/".to_string(),
    }
}

//fn monsta(x: i64) -> i64 {
//    ((((((((((((((2) * (((((((((((((((5) * (3))) * (((((((((((2) * (5))) * (((4) * (3))))) + (((((((5) * (5))) + (3))) + (((((3) + (((2) * (((((3) * (((1) + (((4) * (7))))))) / (3))))))) + (((5) * (20))))))))) / (3))) - (((8) * (3))))))) * (((((((((18) * (6))) + (((3) * (5))))) + (((((((2) + (((20) - (((3) * (2))))))) * (8))) + (((17) * (3))))))) / (2))))) - (((((((((((((3) * (13))) * (((((((((((17) * (2))) / (2))) * (2))) / (2))) * (5))))) - (((((((((((2) * (((5) * (5))))) - (((1) + (6))))) * (2))) - (((2) + (5))))) * (((1) + (((5) * (2))))))))) / (2))) * (2))) * (4))))) * (2))) * (((((((((((13) * (((((5) * (5))) + (4))))) + (2))) - (((2) * (3))))) * (((((5) * (3))) + (2))))) * (5))))) + (((((((((1) + (6))) * (((5) * (((((((3) * (((((2) * (((((5) + (12))) * (2))))) + (5))))) * (((3) * (2))))) + (((((((7) * (6))) + (((((((((2) * (((7) + (((((((((10) * (((3) * (((4) + (((3) + (4))))))))) / (2))) - (14))) - (7))))))) / (2))) + (((3) * (((17) * (2))))))) - (((((5) * (3))) + (((13) * (3))))))))) * (7))))))))) * (((((((((((2) * (((5) + (((2) * (((((3) * (3))) - (3))))))))) * (((2) * (14))))) + (((((((((3) * (((((((((5) + (2))) * (2))) + (((((((((((((((2) * (((((5) * (((4) * (4))))) - (((((5) * (2))) + (((1) + (10))))))))) / (2))) + (((15) * (2))))) + (((11) * (3))))) / (2))) - (15))) / (2))))) - (3))))) + (1))) * (3))) * (2))))) + (((17) * (8))))) * (((((((3) * (3))) - (2))) + (((((4) + (((5) * (5))))) + (((4) * (8))))))))))) + (((((((((((((((((((((4) + (((17) - (4))))) * (((((3) * (((((((5) * (2))) * (3))) - (((((5) * (2))) - (3))))))) / (3))))) * (2))) + (((((3) * (((((5) * (5))) * (5))))) + (((11) * (((17) * (2))))))))) * (3))) + (((((((19) + (10))) * (((((((10) * (3))) + (1))) - (2))))) - (((6) + (((((((1) + (((((((2) * (((4) + (3))))) / (2))) * (3))))) + (((((5) * (3))) + (((((2) * (3))) * (5))))))) * (2))))))))) - (((((((((((2) * (((3) + (((5) * (2))))))) / (2))) * (((16) * (3))))) + (((((5) * (5))) * (((((3) * (9))) + (2))))))) + (((((2) * (((((((((7) + (((1) + (10))))) + (((5) * (5))))) + (((2) * (((2) * (((6) + (((12) + (8))))))))))) - (((((((((3) * (3))) * (4))) - (11))) + (((((1) + (6))) * (3))))))))) * (2))))))) * (((((((11) + (6))) * (((2) * (((((2) * (((5) + (((((3) * (2))) * (3))))))) + (((5) * (5))))))))) + (((((((2) * (((((((13) * (2))) / (2))) * (2))))) + (((((6) + (5))) * (((3) + (14))))))) * (((5) * (5))))))))) + (((4) * (((((((((4) * (((((((((2) * (3))) + (1))) * (7))) - (12))))) * (((((18) * (((((((4) * (3))) + (((((5) + (5))) + (((((2) + (((2) * (12))))) + (((6) + (5))))))))) * (2))))) - (13))))) + (((((((((((((((2) * (((((2) * (((((5) * (5))) + (4))))) + (15))))) / (2))) * (3))) * (((10) + (1))))) + (((2) * (((2) * (((((((4) * (4))) * (17))) + (((((((10) + (1))) * (5))) - (10))))))))))) + (((((2) * (((2) * (((((4) * (((2) + (((((3) * (3))) - (2))))))) + (((19) * (5))))))))) * (((((2) * (((((((((((((((2) + (((4) + (((((2) * (3))) + (3))))))) + (((((((((3) + (4))) + (18))) + (((3) * (3))))) + (((((((((1) + (((2) * (11))))) * (2))) / (2))) + (6))))))) / (((((5) + (2))) - (1))))) * (2))) + (8))) * (2))) / (4))))) * (2))))))) * (14))))) / (2))))))) * (((((((((((((2) + (((2) + (4))))) + (((3) * (2))))) + (5))) * (6))) - (((((1) + (((2) * (3))))) * (4))))) + (((((((4) + (20))) + (13))) * (5))))))))))))) * (((((2) * (((4) + (((6) + (7))))))) * (((5) * (5))))))) - (((((((((12) + (((5) * (5))))) * (13))) + (((((((((((2) * (((((((((3) * (((3) + (10))))) * (5))) + (((2) * (((((((3) * (2))) * (2))) + (11))))))) + (((((3) * (3))) * (((((((((((((((((((((4) * (8))) * (((3) * (9))))) + (((((((((((7) * (((((((((((((((((((((((((((4) * (2))) + (2))) * (2))) + (((((((12) * (7))) + (((((((7) + (5))) + (((((((((((10) + (3))) * (3))) * (2))) / (6))) * (7))))) + (((((9) * (2))) * (5))))))) * (3))))) + (((((((((((((((((((2) * (((((((((10) * (3))) + (16))) * (((3) * (7))))) + (((2) * (((((((((1) + (((5) * (((((3) + (4))) + (1))))))) * (2))) + (((((4) * (((2) * (((6) + (((8) + (9))))))))) - (((((16) + (((((((((6) + (7))) * (((((((1) + (((4) * (4))))) + (((4) + (((2) * (3))))))) - (6))))) / (3))) / (7))))) + (((6) * (3))))))))) + (((((((((4) * (((((7) * (5))) + (((2) * (((5) + (((4) * (2))))))))))) + (((((((2) * (((3) * (3))))) * (16))) + (((13) * (((2) * (((((5) * (2))) + (1))))))))))) + (((((5) * (((((((((((((((((((((((((((((((13) * (((6) * (11))))) + (((((x) - (5))) / (2))))) * (((((((5) * (5))) + (((((3) * (3))) + (3))))) + (((4) * (6))))))) - (((((((((((2) * (5))) * (15))) / (2))) + (((((5) * (5))) + (((((12) * (2))) + (((19) * (3))))))))) + (((((((2) * (((((5) + (2))) * (2))))) + (1))) + (((((9) * (((5) + (2))))) + (((2) * (((((((8) + (((5) * (((18) + (1))))))) * (2))) / (2))))))))))))) / (3))) + (((((((2) * (17))) * (11))) + (((14) * (((2) + (((4) + (7))))))))))) * (2))) - (((((((((((5) * (19))) + (((((2) * (12))) + (((5) * (((((2) * (13))) / (2))))))))) + (((((((((20) + (((((((2) + (((12) - (3))))) * (2))) + (((5) * (5))))))) * (4))) + (((((16) - (3))) * (3))))) - (((((4) * (5))) + (((((((4) + (((3) * (3))))) * (2))) + (17))))))))) * (2))) / (2))))) * (2))) + (((((3) * (13))) * (((((((((((4) * (((5) + (((2) * (4))))))) / (2))) - (4))) * (2))) / (2))))))) / (2))) + (((((3) * (17))) + (((((((((((14) + (((((3) + (10))) * (3))))) * (2))) / (2))) * (6))) + (((17) * (((6) + (17))))))))))) + (((2) * (((((((11) * (3))) + (((((((5) + (5))) + (3))) * (2))))) + (((((1) + (((3) * (3))))) * (((4) + (((5) * (((3) + (4))))))))))))))) / (3))) - (((((5) * (((((5) + (2))) + (4))))) * (7))))))) - (((((((2) * (((((((((((14) + (9))) + (10))) * (5))) + (((((2) * (((((((((((5) + (6))) * (2))) + (19))) * (3))) + (((((2) * (((14) - (3))))) + (6))))))) * (2))))) + (((2) * (14))))))) * (2))) / (4))))))) / (3))))))))))) - (((((((((7) * (13))) * (5))) + (((2) * (4))))) + (((4) * (((((14) * (3))) + (19))))))))) / (3))) - (((2) * (((((((((5) * (11))) + (((3) * (2))))) * (3))) + (((16) / (2))))))))) * (2))) + (((((((2) * (((((((5) * (((1) + (6))))) + (((2) * (((1) + (8))))))) + (8))))) + (((3) * (((((((((2) * (3))) * (2))) + (((((3) + (6))) + (((7) + (1))))))) * (3))))))) * (2))))) / (4))) - (((((9) * (3))) + (((2) * (((((11) + (((((((3) * (((((4) * (20))) - (((((((2) * (3))) + (3))) + (((5) * (2))))))))) - (((((((((((7) + (((13) + (6))))) * (4))) / (4))) / (2))) * (3))))) / (2))))) * (2))))))))) * (3))))) / (10))) - (((2) + (((8) + (((15) + (6))))))))) * (4))) + (((((((((5) + (1))) + (11))) * (((((2) * (7))) + (((3) * (5))))))) + (((((((((8) * (5))) + (((((2) * (4))) + (((5) + (18))))))) * (2))) * (2))))))) / (5))) - (((((19) * (5))) * (((3) + (4))))))) / (2))) + (((((((((2) * (5))) - (2))) + (((2) * (3))))) * (((4) * (((4) * (2))))))))))) + (((((3) * (9))) * (4))))) / (8))) - (((((((15) * (8))) + (((((2) * (((((17) + (12))) - (((3) * (2))))))) + (1))))) * (3))))) * (((2) + (((13) * (3))))))))) + (((((((7) * (11))) + (((((((((7) * (3))) + (((((((5) + (((8) * (3))))) + (5))) * (2))))) + (((((3) * (((((5) + (2))) * (2))))) + (((((2) * (4))) * (13))))))) * (2))))) + (((((2) * (((((13) + (16))) + (10))))) + (12))))))) / (2))) - (((((1) + (6))) * (((((12) + (((((((3) * (((((6) + (1))) + (16))))) - (1))) - (((3) * (((3) + (4))))))))) + (((((((4) * (((9) + (1))))) + (3))) + (11))))))))) * (2))) + (((((((((((((2) * (((((((3) * (((((((((7) * (2))) + (17))) * (2))) / (2))))) * (3))) + (((((((5) * (((12) + (1))))) + (((((((3) * (((13) + (((3) * (((4) * (2))))))))) + (((5) + (2))))) + (((((((((2) * (5))) * (((4) * (2))))) + (((((((((((3) * (3))) * (2))) * (((3) * (2))))) + (((((5) + (2))) * (7))))) + (((4) * (18))))))) + (((17) * (10))))))))) - (((9) * (((((2) * (((3) * (2))))) + (((2) * (4))))))))))))) + (((((((((8) - (2))) * (((3) * (2))))) * (((4) * (4))))) - (((((19) * (5))) + (((9) * (6))))))))) + (((((3) * (((11) + (((2) * (3))))))) * (((6) + (2))))))) + (((13) * (13))))) / (6))) * (2))))) / (9))) - (((((((((((8) * (((11) + (((((2) * (3))) + (7))))))) / (((5) + (3))))) + (11))) + (18))) + (((((5) * (2))) * (((((((13) + (((7) * (4))))) - (12))) + (((5) + (5))))))))))))))))) - (((((((2) * (((((6) * (((((((5) + (2))) + (4))) * (2))))) - (5))))) + (((((2) + (((3) + (((2) * (((3) * (4))))))))) * (13))))) + (((((((3) + (10))) * (3))) * (4))))))) / (((((3) * (2))) + (1))))) - (((((7) * (2))) + (((((((3) * (4))) + (1))) + (((2) * (((((4) + (3))) + (((3) * (2))))))))))))) / (2))))) * (2))))) * (4))) + (((((2) * (((13) * (2))))) * (((((2) * (13))) / (2))))))) / (4))) - (((7) * (((((((((((((((2) + (5))) * (((((((((((((((3) * (9))) * (3))) + (((1) + (((((((5) * (2))) * (((((3) * (8))) - (1))))) / (2))))))) + (((((((((((((((((((3) + (7))) + (((((((((((3) * (2))) * (2))) + (7))) * (2))) / (2))))) * (2))) * (2))) - (((((((3) * (((((((((((13) * (5))) / (5))) * (3))) - (2))) * (2))))) / (2))) / (3))))) + (((((2) * (((3) + (((2) + (((1) + (13))))))))) * (3))))) * (5))) * (2))) - (((((((16) * (((9) + (1))))) / (4))) * (((7) * (2))))))))) + (((((((3) * (((((((((3) * (2))) * (((2) * (((((8) + (3))) + (((4) * (5))))))))) + (((13) * (3))))) + (((((((((4) + (((((2) * (17))) * (2))))) + (((2) * (((7) - (1))))))) + (17))) * (2))))))) * (((((1) + (7))) + (15))))) - (((((((((((4) * (((1) + (((1) + (5))))))) - (3))) + (((7) * (((3) + (3))))))) + (((11) * (2))))) * (((((((3) * (((((6) + (5))) * (3))))) + (4))) + (13))))))))) + (((((5) * (((((((((12) / (2))) + (((((2) * (5))) + (5))))) * (((((7) + (((7) + (((17) * (2))))))) - (((((4) * (((3) * (2))))) / (2))))))) + (((19) * (((((((4) * (((2) + (11))))) + (((((((2) + (5))) * (5))) + (5))))) + (((7) * (3))))))))))) + (((((((((((((2) * (4))) + (((((((5) * (((1) + (((3) * (2))))))) + (((2) + (4))))) + (12))))) + (((((1) + (((((3) + (7))) * (7))))) - (((3) * (((3) * (2))))))))) + (((18) - (5))))) * (((3) * (11))))) + (((((13) * (((((((11) * (2))) + (((2) * (17))))) + (((((4) * (19))) + (((7) * (((((2) * (((3) * (4))))) + (7))))))))))) * (2))))))))) + (((((((((11) * (2))) * (((((((2) * (((((((5) * (3))) + (((((4) * (((1) + (((5) + (1))))))) / (2))))) - (((2) * (3))))))) + (15))) * (5))))) + (((((((((((3) * (((((2) * (((3) * (5))))) + (((3) + (4))))))) + (((2) * (((((5) * (2))) + (13))))))) * (3))) + (((7) * (((15) + (16))))))) + (((((((((((7) + (6))) * (((((((((((3) * (3))) + (8))) - (4))) * (2))) - (7))))) * (3))) + (((((((4) * (14))) / (((3) + (5))))) * (((((((((((((((14) / (2))) * (4))) - (5))) + (((((((3) * (3))) * (8))) + (1))))) + (((3) + (((20) / (2))))))) + (((((((((2) + (((3) * (((5) + (2))))))) * (2))) / (2))) + (((((2) * (3))) * (2))))))) + (7))))))) / (2))))))) * (19))))))) + (((((((((((2) * (((((((((7) * (3))) * (((((3) * (14))) + (((5) * (2))))))) / (7))) + (((((14) - (1))) + (12))))))) * (2))) + (((18) + (((((((((8) * (((4) * (2))))) + (((((5) + (((((2) * (3))) * (2))))) * (((5) * (2))))))) - (((((((5) * (3))) + (17))) - (3))))) - (11))))))) + (((2) * (((((2) * (((5) * (((((((3) * (4))) + (((((3) * (3))) * (6))))) + (((((3) * (2))) + (((1) + (6))))))))))) + (((((((9) - (2))) * (3))) * (3))))))))) * (((((((((17) + (20))) * (3))) * (3))) / (3))))))) + (((5) * (((((((1) + (10))) * (((((((((((3) * (((((((((2) * (((((((1) + (((((5) * (5))) + (10))))) + (5))) + (((2) * (((8) * (2))))))))) / (2))) * (3))) - (8))))) * (3))) + (((((15) + (14))) * (((((5) * (13))) - (4))))))) / (2))) / (2))))) + (((3) * (((((((((13) + (10))) + (((((((6) * (2))) + (2))) * (6))))) * (((((4) * (5))) * (10))))) - (((((((((((((((((18) * (5))) + (((((5) * (2))) * (7))))) - (((7) * (((((15) + (20))) / (5))))))) / (3))) + (((((4) * (2))) * (3))))) * (5))) + (((((5) * (5))) * (5))))) + (((((9) - (2))) * (((((((10) - (3))) * (12))) + (((((4) * (((5) * (5))))) + (((3) + (((((8) * (4))) * (2))))))))))))))))))))))) * (5))) * (((((((((((((17) + (((4) * (14))))) - (((6) * (2))))) + (((((9) * (3))) + (((7) * (3))))))) * (((((((((4) * (((((3) * (5))) + (((((2) * (4))) * (((3) + (4))))))))) * (((((3) + (3))) + (2))))) / (8))) - (((5) * (11))))))) * (2))) + (((((((((((((((3) * (3))) + (3))) + (((3) * (9))))) * (((14) * (((11) + (2))))))) / (2))) / (((2) + (5))))) * (((5) * (((((((2) * (((((3) * (3))) + (2))))) + (((10) * (3))))) + (((((4) * (((((7) + (((((((2) * (((4) + (((6) + (1))))))) + (1))) + (((6) + (1))))))) * (2))))) / (((3) + (5))))))))))))))) + (((((((((3) + (10))) * (((((((((((((5) * (5))) + (2))) + (2))) + (((2) * (10))))) * (((2) * (3))))) + (((((5) * (2))) + (((((6) * (7))) + (1))))))))) * (3))) * (((((((((((((((((((((((((3) * (2))) + (((3) + (((((1) + (((3) * (2))))) * (2))))))) * (19))) - (((((((1) + (((((7) + (3))) * (2))))) * (4))) + (((((((3) * (3))) + (2))) * (5))))))) + (((((((2) + (((3) * (17))))) * (3))) + (((((((8) + (5))) * (((4) * (2))))) - (4))))))) * (3))) * (((((((3) * (3))) + (3))) + (2))))) - (((((((((((((((5) * (((8) + (2))))) + (11))) + (((7) * (((1) + (((4) * (2))))))))) / (4))) * (2))) + (((5) * (((1) + (((2) * (6))))))))) * (((1) + (((12) * (5))))))))) + (((2) * (((((((2) * (((((((((((3) + (((10) + (((2) * (5))))))) + (8))) * (2))) - (((5) + (2))))) + (((2) * (((((12) + (3))) + (((14) * (4))))))))))) + (((((1) + (((2) * (5))))) * (5))))) * (2))))))) * (((((2) * (((12) + (((3) * (((5) * (3))))))))) - (17))))) + (((((((((((((((((((3) * (((((4) * (2))) + (3))))) * (((((((5) * (2))) * (4))) - (((10) + (2))))))) / (2))) + (((((((11) * (((10) + (11))))) + (((3) * (((((((2) * (13))) / (2))) - (4))))))) - (((((3) * (11))) + (((7) * (2))))))))) * (((((5) + (((4) * (((20) + (4))))))) + (((((((13) - (4))) * (4))) + (6))))))) + (((((((((((((((2) + (((((2) * (4))) + (3))))) * (6))) + (((5) * (5))))) + (((((((6) + (7))) + (13))) * (5))))) - (((4) * (((((3) + (((2) * (5))))) + (((1) + (5))))))))) * (((((((((((((2) * (((((2) * (((2) * (3))))) + (((5) + (((2) * (13))))))))) + (((((4) * (2))) + (3))))) * (3))) + (((((((4) * (4))) + (7))) - (4))))) + (((((6) + (1))) * (4))))) + (((14) * (4))))))) + (((((((((((3) * (5))) + (((14) * (2))))) + (((2) * (13))))) * (2))) * (((5) * (((9) + (((4) * (2))))))))))))) + (((5) * (((4) * (((((((((((((((((((((((4) * (((2) * (((((((4) * (((2) * (((((((14) + (((((2) * (((((2) + (6))) + (9))))) / (2))))) * (3))) / (3))))))) - (((13) * (2))))) / (((5) + (1))))))))) + (((((3) * (((((((((6) + (8))) * (4))) + (((2) * (((((9) - (2))) * (2))))))) - (5))))) * (((((3) * (3))) + (10))))))) * (2))) / (2))) * (7))) / (((3) + (4))))) * (2))) / (2))) * (4))) / (4))) + (((((2) + (((((((((((((((((((((1) + (((3) * (2))))) * (11))) + (((((((20) + (((3) * (8))))) + (((((((2) * (7))) + (((3) * (3))))) - (((2) * (3))))))) * (2))))) - (((((((1) + (((((3) * (6))) / (3))))) * (((2) * (((3) * (3))))))) / (3))))) * (2))) - (2))) + (4))) / (4))) * (5))) + (((13) * (((((5) + (((7) * (3))))) / (2))))))))) * (7))))))))))) * (2))) * (4))))) * (2))))))))))
//}