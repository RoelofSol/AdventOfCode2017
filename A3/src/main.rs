// https://adventofcode.com/2017/day/3

use std::collections::*;
fn main() {
   
    let input = 265149;

    // Cordinates (N-2,-(N-2)) == N^2 for all uneven N
    let mut sqrt = (input as f64).sqrt().floor() as i32;
    if sqrt %2 == 0 {
        sqrt -= 1;
    };
    let mut coordinates = SpiralCoordinates{
        x : sqrt/2 , y: -sqrt/2
    };
    let mut nth_square = sqrt*sqrt;
    while nth_square != input {
        coordinates.next().unwrap();
        nth_square += 1;
    }
    

    println!("{:?} {} ",coordinates, coordinates.x.abs() + coordinates.y.abs());

    // Part 2 
    let mut map : HashMap<(i32,i32),u32> = HashMap::new();
    map.insert((0,0),1);
    let coordinates = SpiralCoordinates{
        x : 1 , y: 0
    };
    for (x,y) in coordinates {
        let mut sum = 0;
        sum += *map.get(&(x-1,y)).unwrap_or(&0);
        sum += *map.get(&(x+1,y)).unwrap_or(&0);
        sum += *map.get(&(x,y-1)).unwrap_or(&0);
        sum += *map.get(&(x,y+1)).unwrap_or(&0);
        sum += *map.get(&(x-1,y+1)).unwrap_or(&0);
        sum += *map.get(&(x+1,y-1)).unwrap_or(&0);
        sum += *map.get(&(x-1,y-1)).unwrap_or(&0);
        sum += *map.get(&(x+1,y+1)).unwrap_or(&0);
        map.insert((x,y),sum);

        if sum as i32 > input  {
            println!("{}", sum);
            break;
        }
    }
}

#[derive(Debug)]
struct SpiralCoordinates{
    x:i32,y:i32
}
impl Iterator for SpiralCoordinates {
    type Item = (i32,i32);
    fn next(&mut self) -> Option<Self::Item> {
        let res = (self.x,self.y);
        if self.x >= 0 && self.x == -self.y {
            self.x += 1;
        }else {
            let side = i32::max(self.x.abs(), self.y.abs());
            match (to_side(side, self.x), to_side(side, self.y)) {
                (Positive, Inbetween) => self.y += 1,
                (Positive, Positive) | (Inbetween, Positive) => self.x -= 1,
                (Negative, Positive) | (Negative, Inbetween) => self.y -= 1,
                (_, Negative) => self.x += 1,
                _ => panic!()
            }
        }


        Some(res)
    }
}
use self::Side::*;
enum Side {
    Positive ,
    Negative,
    Inbetween
}
fn to_side(side:  i32 , i : i32 ) -> Side {
    if side == i {
        Positive
    } else if -side == i { Negative } else {Inbetween}
}