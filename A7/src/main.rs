
// https://adventofcode.com/2017/day/7

#[macro_use]
extern crate nom;
use nom::*;
use std::str;
use std::collections::*;

fn main() {

    let input = nodes(include_str!("../inp.txt").as_bytes())
        .to_result()
        .unwrap();

    let mut parent_lookup: HashMap<&str, &str> = HashMap::new();
    for node in input.iter() {
        for child_name in node.support.iter() {
            parent_lookup.insert(*child_name, node.name);
        }
    }

    let mut nodes_by_name: HashMap<_, _> = input
        .into_iter()
        .map(|mut n| {
            n.parent = parent_lookup.get(n.name).cloned(); // Set parent name
            (n.name, n)
        })
        .collect();


    let root_node = {
        nodes_by_name
            .values()
            .find(|v| v.parent.is_none())
            .unwrap()
            .name
            .clone()
    };

    println!("{:?}", root_node);

    // Part 2

    println!("{:?}", inspect(&root_node, &mut nodes_by_name));
}

fn inspect(name: &str, hmap: &HashMap<&str, Node>) -> Result<u32, (String, u32)> {
    let (weight, children) = {
        let tmp = hmap.get(name).unwrap();
        (tmp.weight, tmp.support.clone())
    };

    let mut sub_values: HashMap<u32, Vec<&str>> = HashMap::new();

    let mut total_sub_weight = 0;

    for child in children {
        let val = inspect(child, hmap)?;
        total_sub_weight += val;
        let n: &mut Vec<_> = sub_values.entry(val).or_insert_with(|| vec![]);
        n.push(child);
    }

    if sub_values.len() <= 1 {
        return Ok(weight + total_sub_weight);
    }

    let mut weight_and_children: Vec<_> = sub_values.into_iter().collect();

    weight_and_children.sort_unstable_by(|a, b| a.1.len().cmp(&b.1.len()));


    let &(ref current_weight, ref unbalanced_nodes_list) = &weight_and_children[0];
    let &(ref target_weight, _) = &weight_and_children[1];

    let unbalanced = unbalanced_nodes_list[0];
    let weight = hmap.get(unbalanced).unwrap().weight;

    Err((
        unbalanced.to_string(),
        weight + target_weight - current_weight,
    ))
}

#[derive(Debug)]
struct Node<'a> {
    name: &'a str,
    weight: u32,
    support: Vec<&'a str>,
    parent: Option<&'a str>,
}


named!(nodes < Vec<Node>> , ws!(many1!(node)));
named!(node <Node> , ws!( do_parse!( 
    name : name
    >> weight : weight 
    >> children : opt!( children)
    >> ( Node { name , weight , support : children.unwrap_or(vec![]) ,parent: None})
 ) ) );

named!(children <Vec<&str> > , ws!( do_parse!( 
     tag!("->") 
     >> names : separated_list_complete!(tag!(", "), name) 
     >> (names) 
     )
 ));

named!(name<&str>, map_res!(alphanumeric, str::from_utf8));
named!(weight<u32>, map_res!(
        map_res!(
            delimited!(
                char!('('),
                is_not!(")"),
                char!(')')
            ),
            str::from_utf8
        ),
        str::parse
));