//https://adventofcode.com/2017/day/6

use std::collections::*;

const STATE_SIZE: usize = 16;
fn main() {

    let mut set: HashMap<[u8; STATE_SIZE], u32> = HashMap::new();
    let mut state: [u8; STATE_SIZE] = [4, 1, 15, 12, 0, 9, 9, 5, 5, 8, 7, 3, 14, 5, 12, 3];
    let mut i = 0;

    while set.get(&state).is_none() {
        set.insert(state, i);
        i += 1;
        let max: u8 = *state.iter().max().unwrap();
        let idx: usize = state.iter().position(|i| *i == max).unwrap();
        state[idx] = 0;
        let mut distribute = max;
        let mut at = idx as usize;

        while distribute > 0 {
            at = (at + 1) % STATE_SIZE;
            if at != idx  {
                state[at] += 1;
                distribute -= 1;
            }

        }
    }

    println!("Set size {}", set.len());

    // Part 2
   
    println!("Cycle size {:?}", i - set.get(&state).unwrap() );
 
}