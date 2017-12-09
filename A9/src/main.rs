// http://adventofcode.com/2017/day/9

fn main() {
    let it = &mut include_str!("../inp.txt").chars().peekable();

    let (group_score, garbage_count) = parse(it, 1);
    assert!(it.next().is_none());

    println!("{}", group_score);

    // Part 2

    println!("{}", garbage_count);
}

type It = std::iter::Peekable<std::str::Chars<'static>>;

fn parse(input: &mut It, score: u32) -> (u32, u32) {
    match input.peek() {
        Some(&'{') => parse_group(input, score),
        Some(&'<') => parse_garbage(input),
        _ => (0, 0),
    }
}

fn parse_group(input: &mut It, level: u32) -> (u32, u32) {
    assert_eq!(input.next().unwrap(), '{');

    let (mut score, mut garbage_count) = parse(input, level + 1);
    while let Some(&',') = input.peek() {
        input.next();
        let (s_score, s_garbage) = parse(input, level + 1);
        score += s_score;
        garbage_count += s_garbage;
    }
    assert_eq!(input.next().unwrap(), '}');
    (score + level, garbage_count)
}

fn parse_garbage(input: &mut It) -> (u32, u32) {
    assert_eq!(input.next().unwrap(), '<');
    let mut garbage_count = 0;
    loop {
        match input.next() {
            Some('>') => return (0, garbage_count),
            Some('!') => {
                input.next();
            }
            _ => garbage_count += 1,
        }
    }
}
