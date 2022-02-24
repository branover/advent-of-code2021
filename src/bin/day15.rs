#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use advent::read_lines;
use petgraph::visit::EdgeRef;
use std::fmt::Display;
use std::{fmt, io::BufRead};
use std::collections::{HashMap};
use std::time::{Duration, Instant};
use itertools::Itertools;
use petgraph::graph::{NodeIndex, UnGraph, Graph};
use petgraph::Undirected;
use petgraph::algo::{dijkstra, bellman_ford};

fn calculate_node_index(x: usize, y: usize, num_columns: usize) -> NodeIndex {
    NodeIndex::new((y * num_columns) + x)
}

fn day15_1() {
    let array = include_bytes!("../../inputs/day15_input")
        .split(|&b| b == b'\n')
        .map(|l| l.iter().map(|c| (*c as char).to_digit(10).unwrap()).collect::<Vec<u32>>())
        .collect::<Vec<_>>();
    let mut graph = Graph::<u32,u32,Undirected>::new_undirected();
    for y in 0..array.len() {
        for x in 0..array[0].len() {
            // println!("{}", array[y][x]);
            graph.add_node(array[y][x]);
            match array.get((y as isize -1) as usize).and_then(|l| l.get(x)) {
                Some(o) => {
                    graph.add_edge(
                        calculate_node_index(x,y,array[0].len()),
                        calculate_node_index(x,y-1, array[0].len()),
                    1
                    );
                },
                None => (),
            };
            match array.get(y).and_then(|l| l.get((x as isize -1) as usize)) {
                Some(o) => {
                    graph.add_edge(
                        calculate_node_index(x,y,array[0].len()),
                        calculate_node_index(x-1,y, array[0].len()),
                    1
                    );
                },
                None => (),
            };
        }
    }
    let max_node_id = calculate_node_index(array[0].len() -1, array.len() -1, array[0].len());
    println!("{:?}", max_node_id);
    let res = dijkstra(&graph,
        NodeIndex::new(0),
        Some(max_node_id),
        |e| {
            graph[e.target()]
        }
    );
    println!("{:?}",res[&max_node_id]);
}


fn day15_2() {
    let init_array = include_bytes!("../../inputs/day15_input")
        .split(|&b| b == b'\n')
        .map(|l| l.iter().map(|c| (*c as char).to_digit(10).unwrap()).collect::<Vec<u32>>())
        .collect::<Vec<_>>();
    let array = init_array.iter()
        .enumerate()
        .fold(vec![vec![0; init_array[0].len()*5]; init_array.len()*5], |mut v, (row_i, row)| {
        for i in 0..5 {
            for k in 0..5 {
                row.iter().enumerate().for_each(|(j,e)| {
                    let mut new_val: u32 = (*e + (i + k) as u32) % 10;
                    if new_val < *e {
                        new_val += 1;
                    }
                    v[row_i + init_array.len() * k][j + (row.len() * i)] = new_val;
                });                
            }
        }
        v
    });
    // println!("{:?}", array[array.len() - 1]);

    let mut graph = Graph::<u32,u32,Undirected>::new_undirected();
    let max_x = array[0].len();
    let max_y = array.len();
    for y in 0..max_y {
        for x in 0..max_x {
            // println!("{}", array[y][x]);
            graph.add_node(array[y][x]);
            match array.get((y as isize -1) as usize).and_then(|l| l.get(x)) {
                Some(o) => {
                    graph.add_edge(
                        calculate_node_index(x,y,max_x),
                        calculate_node_index(x,y-1, max_x),
                    1
                    );
                },
                None => (),
            };
            match array.get(y).and_then(|l| l.get((x as isize -1) as usize)) {
                Some(o) => {
                    graph.add_edge(
                        calculate_node_index(x,y,max_x),
                        calculate_node_index(x-1,y, max_x),
                    1
                    );
                },
                None => (),
            };
        }
    }
    let max_node_id = calculate_node_index(max_x-1, max_y-1, max_x);
    println!("{:?}", max_node_id);
    let res = dijkstra(&graph,
        NodeIndex::new(0),
        Some(max_node_id),
        |e| {
            graph[e.target()]
        }
    );
    println!("{:?}",res[&max_node_id]);
}

fn main() {
    let start = Instant::now();
    day15_2();
    let duration = start.elapsed();

    println!("Time elapsed is: {:?}", duration);
}