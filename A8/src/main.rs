

// https://adventofcode.com/2017/day/9


#[macro_use]
extern crate nom;
use nom::*;
use std::str;
use std::collections::*;

fn main() {
    let input = instructions(include_str!("../inp.txt").as_bytes())
        .to_result()
        .unwrap();

    let mut state = HashMap::new();
    let mut all_time_max = 0;
    for inst in input.into_iter() {
        inst.operate(&mut state, &mut all_time_max);
    }

    println!("Current max {:?}", state.values().max().unwrap());

    // Part 2

    println!("All time max {:?}", all_time_max);
}

type Operation = Box<Fn(i32) -> Option<i32>>;

struct Instr<'a> {
    operation: Operation,
    target: &'a str,
    conditional: &'a str,
}
impl<'a> Instr<'a> {
    fn operate(&self, map: &mut HashMap<&'a str, i32>, all_time_max: &mut i32) {
        let conditional_val = map.get(&self.conditional).cloned().unwrap_or_default();
        if let Some(diff) = (self.operation)(conditional_val) {
            let val: &mut i32 = map.entry(&self.target).or_insert(0);
            *val += diff;
            *all_time_max = i32::max(*all_time_max, *val);
        }
    }
}

named!(instructions <Vec<Instr>> , ws!( many0!( instr )));

named!(instr <Instr>,  ws!( do_parse!(
    target:  not_space  >>
    inc: dbg_dmp!( is_inc )  >>
    target_diff : dbg_dmp!( integer )>>
    tag!("if") >>
    conditional : dbg_dmp!( not_space )>>
    test_op : dbg_dmp!( not_space ) >>
    conditional_value : dbg_dmp!( integer)  >>
    (Instr { 
            conditional,
            target,
            operation : build_operation(inc , target_diff, test_op , conditional_value)
    })
  )
) ) ;


named!( not_space <&str> , map_res!( take_while1!(|c| !(c as char).is_whitespace() ) , str::from_utf8) );
named!( integer <i32> ,  map_res!( map_res!(  take_while1!(|c| c == b'-' || (c as char).is_digit(10) ) ,str::from_utf8), str::parse) ) ;
named!( is_inc <bool> , alt!( map!( tag!("inc") , |_| true ) | map!( tag!("dec")   , |_| false ) ));


type TestOp = &'static Fn(i32,i32) -> bool;

fn build_operation(inc: bool, mut diff: i32, if_op: &str, if_val: i32) -> Operation {
    if !inc {
        diff = diff * -1;
    }
    let if_func = match if_op {
        "==" => &eq as TestOp,
        "!=" => &neq as TestOp,
        "<" => &lt as TestOp,
        "<=" => &lte as TestOp,
        ">" => &gt as TestOp,
        ">=" => &gte as TestOp,
        e => unimplemented!("Op {} is not implemented",e),
    };

    Box::new(move |v| if if_func(v,if_val) { Some(diff) } else { None })
}

fn eq(a: i32, b: i32) -> bool { a == b }
fn neq(a: i32, b: i32) -> bool { a != b }
fn lt(a: i32, b: i32) -> bool { a < b }
fn lte(a: i32, b: i32) -> bool { a <= b }
fn gt(a: i32, b: i32) -> bool { a > b }
fn gte(a: i32, b: i32) -> bool { a >= b }