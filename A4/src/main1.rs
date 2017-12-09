use std::iter::FromIterator;
use std::collections::*;
fn main() {
    let input = include_str!("../inp.txt");

    let assignment1 = input.lines().map(n_unique_words).sum();

    println!("{}", assignment1);

    let set = parse(input);
    println!("input {:?}", set);
}
fn n_unique_words(line: &str) -> u32 {
    let v: Vec<_> = line.split(' ').collect();
    let s: HashSet<_> = v.iter().collect();
    if s.len() == v.len() { 1 } else { 0 }
}
