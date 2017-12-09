const SIZE: usize = 16;

const START_STATE: [u8; SIZE] = [4, 1, 15, 12, 0, 9, 9, 5, 5, 8, 7, 3, 14, 5, 12, 3];


fn main() {

    let mut set: HashSet<[u8; SIZE]> = HashSet::new();

    let mut state = START_STATE.clone();
    //let mut state  : [u8;SIZE] = [0,2,7,0];
    let mut i = 0;
    while (set.insert(state)) {
        i += 1;
        let max: u8 = *state.iter().max().unwrap();
        let idx: usize = state.iter().position(|i| *i == max).unwrap();
        state[idx] = 0;
        let mut distribute = max;
        let mut at = idx as usize;

        while (distribute > 0) {
            at = (at + 1) % SIZE;
            if at == idx {

            } else {
                state[at] += 1;
                distribute -= 1;
            }

        }
    }
    println!("{:?}", set.len());
}
