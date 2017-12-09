// https://adventofcode.com/2017/day/1 

fn main() {

    let digits = include_str!("../inp.txt")
        .chars()
        .flat_map(|i| char::to_digit(i, 10))
        .collect::<Vec<u32>>();
    let size = digits.len();

    let assignment1: u32 = digits
        .iter()
        .enumerate()
        .map(|(i, a)| if *a == digits[(i as usize + 1) % size] {
            *a
        } else {
            0
        })
        .sum();
    println!("{}", assignment1);

    // Part 2

    let step = size / 2;
    let assignment2: u32 = digits
        .iter()
        .enumerate()
        .map(|(i, a)| if *a == digits[(i as usize + step) % size] {
            *a
        } else {
            0
        })
        .sum();

    println!("{}", assignment2);
}
