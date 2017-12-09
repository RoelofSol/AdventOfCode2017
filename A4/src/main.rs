//https://adventofcode.com/2017/day/4

use std::collections::*;
fn main() {
    let input = include_str!("../inp.txt");

    let assignment1: u32 = input.lines().map(n_unique_words).sum();

    println!("{}", assignment1);

    let assignment2: u32 = input.lines().map(n_unique_letter_sets).sum();
    println!("{:?}", assignment2);
}
fn n_unique_words(line: &str) -> u32 {
    let v: Vec<_> = line.split(' ').collect();
    let s: HashSet<_> = v.iter().collect();
    if s.len() == v.len() { 1 } else { 0 }
}

fn n_unique_letter_sets(line: &str) -> u32 {
    let v: Vec<_> = line.split(' ').map(order).collect();
    let s: HashSet<_> = v.iter().collect();
    if s.len() == v.len() { 1 } else { 0 }
}

fn order(s: &str) -> Vec<char> {
    let mut r = s.chars().collect::<Vec<char>>();
    r.sort_unstable();
    r
}
