//https://adventofcode.com/2017/day/5

fn main() {
    let mut input = parse(include_str!("../inp.txt"));

    let mut i: i32 = 0;
    let mut n = 0;
    while i >= 0 && i <= input.len() as i32 - 1 {
        let offset = input[i as usize];
        input[i as usize] += 1;
        i += offset as i32;
        n += 1;
    }

    println!("{:?}", n);

    // Part 2
    let mut input = parse(include_str!("../inp.txt"));
    let mut i: i32 = 0;
    let mut n = 0;
    while i >= 0 && i <= input.len() as i32 - 1 {
        let offset = input[i as usize];
        if offset >= 3 {
            input[i as usize] -= 1;
        } else {
            input[i as usize] += 1;
        }
        i += offset as i32;
        n += 1;
    }

    println!("{:?}", n);
}

fn parse(s: &str) -> Vec<i32> {
    s.lines().flat_map(str::parse).collect()
}
