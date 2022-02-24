#![allow(dead_code)]
#![allow(unused_imports)]
use advent::read_lines;
use std::{fmt, io::BufRead};
use std::collections::HashMap;
use std::time::{Duration, Instant};

fn day7_1() {
    let mut lines = read_lines("inputs/day7_input");
    let crabs = lines.next().unwrap().unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<isize>>();
    let max = crabs.iter().max().unwrap();
    let solution = (0..=*max/2)
        .map(|i| {crabs.iter()
                 .fold(0,|total, crab| total + (*crab - i).abs())
        }).min().unwrap();
    println!("{}", solution);
}

fn day7_2() {
    let mut lines = read_lines("inputs/day7_input");
    let crabs = lines.next().unwrap().unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<isize>>();
    let max = crabs.iter().max().unwrap();
    let sums: Vec<isize> = (0..=*max).map(|i| (i*(i+1))/2).collect();
    let solution = (0..=*max/4)
        .map(|i| {crabs.iter()
                 .fold::<isize,_>(0,|total, crab| {
                     let diff: isize = (*crab - i).abs();
                     total + sums[diff as usize]
                 })
        }).min().unwrap();
    println!("{}", solution);
}

fn main() {
    let start = Instant::now();
    day7_2();
    let duration = start.elapsed();

    println!("Time elapsed is: {:?}", duration);
}