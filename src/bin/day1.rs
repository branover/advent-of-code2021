use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::VecDeque;

use advent::read_lines;


fn day1_1() {
    let lines = read_lines("inputs/day1_input");
    let mut prev: i32 = std::i32::MAX;
    let mut solution: i32 = 0;
    for line in lines {
        let line = line.unwrap().parse::<i32>().unwrap();
        if line > prev {
            solution += 1;
        }
        prev = line;
        
    }
    println!("{}", solution);   
}

fn day1_2() {
    let lines = read_lines("inputs/day1_input");
    let mut prev: i32 = std::i32::MAX;
    let mut rolling_vec: VecDeque<i32> = VecDeque::new();
    let mut solution: i32 = 0;
    for line in lines {
        if rolling_vec.len() >= 3 {
            rolling_vec.pop_front();
        }
        let line = line.unwrap().parse::<i32>().unwrap();
        rolling_vec.push_back(line);
        let sum = rolling_vec.iter().sum();
        println!("{:?}",rolling_vec);
        if sum > prev {
            solution += 1;
        }
        prev = sum;
        
    }
    // Minus two because the last two sums don't count
    println!("{}", solution-2);    
}

fn main() {

}