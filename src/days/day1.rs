pub fn part_1(input: &str) -> String {
    let (m, x) =
        input.lines()
            .fold((0u32, 0u32), |(m, acc), x| {
                match x.parse::<u32>() {
                    Err(_) => (u32::max(m, acc), 0),
                    Ok(x) => (m, acc + x),
                }
            });
    u32::max(m, x).to_string()
}

pub fn part_2(input: &str) -> String {
    let mut cals =
        input.split("\n\n")
        .map(|x| x.lines().map(|n| n.parse::<u32>().unwrap()).sum())
        .collect::<Vec<u32>>();
    cals.sort();
    cals.reverse();
    cals.resize(3, 0u32);
    format!("{}", cals.iter().sum::<u32>())
}
