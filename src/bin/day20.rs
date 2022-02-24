#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use advent::read_lines;
use std::fmt::Display;
use std::{fmt, io::BufRead};
use std::collections::{HashMap};
use std::time::{Duration, Instant};
use itertools::Itertools;

fn day12_1() {
    let solution = include_str!("../../inputs/day11_input")
        .lines()
        
    println!("{}",solution);
}


fn day12_2() {

}

fn main() {
    let start = Instant::now();
    day12_1();
    let duration = start.elapsed();

    println!("Time elapsed is: {:?}", duration);
}