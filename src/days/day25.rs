pub fn part_1(inp: &str) -> String {
    let sn: Vec<Vec<char>> = inp.lines().map(|l| l.chars().rev().collect()).collect();
    sum_snaf(&sn)
}

// need to reverse
fn sum_snaf(sn: &Vec<Vec<char>>) -> String {
    let mut places: Vec<char> = Vec::new();
    let mut carry = 0i64;
    for i in 0..25 {
        let mut sum = carry;
        for s in sn {
            let c = s.get(i);
            match c {
                Some('-') => sum -= 1,
                Some('=') => sum -= 2,
                Some('0') => (),
                Some('1') => sum += 1,
                Some('2') => sum += 2,
                None => (),
                _ => panic!("nawww"),
            }
        }
        carry = sum / 5;
        match sum % 5 {
            0 => places.push('0'),
            1 => places.push('1'),
            2 => places.push('2'),
            3 => {
                carry += 1;
                places.push('=');
            },
            4 => {
                carry += 1;
                places.push('-');
            },
            -1 => places.push('-'),
            -2 => places.push('='),
            -3 => {
                carry -= 1;
                places.push('2');
            },
            -4 => {
                carry -= 1;
                places.push('1');
            },
            _ => panic!("no no no"),
        }
    }
    places.reverse();
    places.into_iter().collect()
}

pub fn part_2(_inp: &str) -> String {
    String::new()
}
