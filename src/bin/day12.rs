#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use advent::read_lines;
use std::fmt::Display;
use std::{fmt, io::BufRead};
use std::collections::{HashMap};
use std::time::{Duration, Instant};
use itertools::Itertools;

#[derive(Debug)]
enum NodeType {
    Repeatable,
    Once,
    N(i32),
}

#[derive(Debug)]
struct Node {
    neighbors: Vec<String>,
    node_type: NodeType,
}

impl Node {
    fn new(node_type: NodeType) -> Node {
        Node {neighbors: Vec::new(), node_type }
    }

    fn add_neighbor(&mut self, neighbor: &str) {
        match self.neighbors.iter().find(|s| *s == neighbor) {
            None => self.neighbors.push(String::from(neighbor)),
            _ => ()
        };
    }
}

fn add_node<'a>(nodes: &mut HashMap::<&'a str, Node>, node: &'a str) {
    match nodes.contains_key(node) {
        true => return,
        false => match node {
            "start" => {nodes.insert(node, Node::new(NodeType::Once));},
            "end" => {nodes.insert(node, Node::new(NodeType::Once));},
            x if x.chars().all(|c| c.is_ascii_uppercase()) => {
                nodes.insert(node, Node::new(NodeType::Repeatable));
            }
            x if x.chars().all(|c| c.is_ascii_lowercase()) => {
                nodes.insert(node, Node::new(NodeType::Once));
            }
            _ => panic!("Uh oh!"),
        }
    }
}

fn dfs(nodes: &HashMap::<&str, Node>, so_far: &Vec<String>, this: &str, to: &str) {
    let mut so_far = so_far.clone();
    so_far.push(String::from(this));
    if this == to {
        println!("Found end: {} {:?}", so_far.len(), so_far);
        return;
    }
    for neighbor in &nodes.get(this).unwrap().neighbors {
        match nodes.get(&neighbor.as_str()).unwrap().node_type {
            NodeType::Once => match so_far.iter().find(|s| *s == neighbor) {
                Some(node) => continue,
                _ => (),
            },
            _ =>(),
        };
        dfs(nodes, &so_far, neighbor, to);
    }
}

fn day12_1() {
    let mut nodes = HashMap::<&str, Node>::new();
    include_str!("../../inputs/day12_input")
        .lines()
        .map(|l| l.split_once('-').unwrap())
        .for_each(|(from,to)| {
            add_node(&mut nodes, from);
            add_node(&mut nodes, to);
            nodes.get_mut(from).unwrap().add_neighbor(to);
            nodes.get_mut(to).unwrap().add_neighbor(from);
        });
    
    let so_far = vec![];
    dfs(&mut nodes, &so_far, "start", "end");
}

fn add_node_2<'a>(nodes: &mut HashMap::<&'a str, Node>, node: &'a str) {
    match nodes.contains_key(node) {
        true => return,
        false => match node {
            "start" => {nodes.insert(node, Node::new(NodeType::Once));},
            "end" => {nodes.insert(node, Node::new(NodeType::Once));},
            x if x.chars().all(|c| c.is_ascii_uppercase()) => {
                nodes.insert(node, Node::new(NodeType::Repeatable));
            }
            x if x.chars().all(|c| c.is_ascii_lowercase()) => {
                nodes.insert(node, Node::new(NodeType::N(2)));
            }
            _ => panic!("Uh oh!"),
        }
    }
}

fn dfs_2(nodes: &HashMap::<&str, Node>, so_far: &Vec<String>, this: &str, to: &str, already_visited_smallcave: bool) -> usize {
    let this_node = nodes.get(this).unwrap();
    let mut already_visited_smallcave = already_visited_smallcave;
    match this_node.node_type {
        NodeType::Once => match so_far.iter().find(|s| *s == this) {
            Some(node) => return 0,
            _ => (),
        },
        NodeType::N(n) => match so_far.iter().filter(|s| *s == this).count() {
            2.. => return 0,
            1 => {
                if already_visited_smallcave {return 0;}
                already_visited_smallcave = true;
            },
            _ => (),
        },
        _ =>(),
    };  
    let mut so_far = so_far.clone();
    so_far.push(String::from(this));

    this_node.neighbors.iter().map(|neighbor| {
        if neighbor == to {1}
        else {
            dfs_2(nodes, &so_far, neighbor, to, already_visited_smallcave)
        }
    }).sum()
}

fn day12_2() {
    let mut nodes = HashMap::<&str, Node>::new();
    include_str!("../../inputs/day12_input")
        .lines()
        .map(|l| l.split_once('-').unwrap())
        .for_each(|(from,to)| {
            add_node_2(&mut nodes, from);
            add_node_2(&mut nodes, to);
            nodes.get_mut(from).unwrap().add_neighbor(to);
            nodes.get_mut(to).unwrap().add_neighbor(from);
        });
    
    let so_far = vec![];
    let solution = dfs_2(&mut nodes, &so_far, "start", "end", false);
    println!("{}", solution);
}

fn main() {
    let start = Instant::now();
    day12_2();
    let duration = start.elapsed();

    println!("Time elapsed is: {:?}", duration);
}