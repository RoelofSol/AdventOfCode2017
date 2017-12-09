
// https://adventofcode.com/2017/day/2

fn main() {
    let rows = parse(include_str!("../inp.txt"));

    let assignment1: u32 = rows.iter()
        .map(|row| row.iter().max().unwrap() - row.iter().min().unwrap())
        .sum();

    println!("{:?}", assignment1);

    // Part 2

    let result: u32 = rows.into_iter().map(|row| row_divisable(&row)).sum();

    println!("{:?}", result);
}

fn row_divisable(row: &Vec<u32>) -> u32 {
    for i in row.iter().cloned() {
        for j in row.iter().cloned() {
            if i != j && i % j == 0 {
                return i / j;
            }
        }
    }
    unreachable!();
}

fn parse_line(line: &str) -> Vec<u32> {
    line.split('\t').flat_map(str::parse).collect()
}

fn parse(s: &str) -> Vec<Vec<u32>> {
    s.lines().map(parse_line).collect()
}