

fn main() {
    let mut input = 265149;
    //input = 27;

    let mut i = 1;
    while( input > (i+2)*(i+2)) {
        i = i+2;
    }
    let side: i32 = i+2;
    let mut diff = input - i*i ;

    let mut xy = (side/2 as i32,-side/2 as i32  + 1);



    println!("{:?}  {:?} {:?} ",diff , side  , xy);

    xy.1 = xy.1 + i32::min(diff,side-1) as i32;
    if ( diff > side ) {
        diff = diff - side
    }

    println!("{:?} {:?} ",diff  , xy);

}


fn get_coordinates(i:u32) -> (u32,u32) {
    unimplemented!()
}

fn get_ring(mut i:u32) -> (u32,u32) {
    i = i -1;
    let mut ring_size = 3;
    let mut next_ring_size = 9;
    while(i >= next_ring_size*next_ring_size) {
        i = i - next_ring_size*next_ring_size;
        ring_size = next_ring_size;
        next_ring_size = ring_size+2;
    }
    (ring_size,i)
}