#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use advent::read_lines;
use std::{fmt, io::BufRead};
use std::collections::{HashMap};
use std::time::{Duration, Instant};
use itertools::Itertools;

fn day10_1() {
    let solution: usize = include_bytes!("../../inputs/day10_input")
        .split(|&b| b == b'\n')
        .map(|l| l.to_vec())
        .map(|v| {
            let mut stack = vec![];
            v.iter().find_map(|c| {
                match c {
                    b'(' | b'[' | b'{' | b'<' => {stack.push(c); None},
                    b')' => if *stack.pop().unwrap() != b'(' {Some(3)} else {None},
                    b']' => if *stack.pop().unwrap() != b'[' {Some(57)} else {None},                    
                    b'}' => if *stack.pop().unwrap() != b'{' {Some(1197)} else {None},
                    b'>' => if *stack.pop().unwrap() != b'<' {Some(25137)} else {None},
                    _ => panic!("incorrect char")                            
                }
            }).unwrap_or(0)
        }).sum();
    println!("{}",solution);   
}


fn day10_2() {
    let mut solution: Vec<usize> = include_bytes!("../../inputs/day10_input")
        .split(|&b| b == b'\n')
        .map(|l| l.to_vec())
        .map(|v| {
            let mut stack = vec![];
            v.iter().find_map(|c| {
                match c {
                    b'(' | b'[' | b'{' | b'<' => {stack.push(c); None},
                    b')' => if *stack.pop().unwrap() != b'(' {Some(0)} else {None},
                    b']' => if *stack.pop().unwrap() != b'[' {Some(0)} else {None},                    
                    b'}' => if *stack.pop().unwrap() != b'{' {Some(0)} else {None},
                    b'>' => if *stack.pop().unwrap() != b'<' {Some(0)} else {None},
                    _ => panic!("incorrect char")                            
                }
            }).unwrap_or_else(|| {
               stack.iter().rev().fold(0, |sum, c| {
                    match c {
                        b'(' => sum * 5 + 1,
                        b'[' => sum * 5 + 2,
                        b'{' => sum * 5 + 3,
                        b'<' => sum * 5 + 4,
                        _ => panic!("incorrect char")
                   }
               }) 
            })
        }).filter(|c| *c > 0)
        .collect::<Vec<_>>();
    solution.sort_unstable();
    println!("{}",solution[solution.len()/2]); 
}

fn main() {
    let start = Instant::now();
    day10_2();
    let duration = start.elapsed();

    println!("Time elapsed is: {:?}", duration);
}