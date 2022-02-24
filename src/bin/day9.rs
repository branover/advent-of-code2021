#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use advent::read_lines;
use std::{fmt, io::BufRead};
use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};
use itertools::Itertools;

fn day9_1() {
    let mut solution = 0;
    let floor = include_str!("../../inputs/day9_input")
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    floor.iter()
        .enumerate()
        .for_each(|(i,r)| {
            r.iter()
                .enumerate()
                .for_each(|(j,c)| {
                    if ((i == 0) || (floor[i][j] < floor[i-1][j])) &&
                    ((i == floor.len()-1) || (floor[i][j] < floor[i+1][j])) &&
                    ((j == 0) || (floor[i][j] < floor[i][j-1])) &&
                    ((j == r.len()-1) || (floor[i][j] < floor[i][j+1])) {
                        solution += 1+floor[i][j].to_digit(10).unwrap();
                    }
                })
        });
    println!("{}", solution);
}

fn check_neighbors(i: usize, j: usize, floor: &Vec<Vec<char>>, set: &mut HashSet<(usize,usize)>) {
    if floor[i][j] == '9' {
        return;
    }
    set.insert((i,j));
    if (i != 0) && (floor[i][j] < floor[i-1][j]) {
        check_neighbors(i-1,j,&floor,set);
    }
    if (i != floor.len()-1) && (floor[i][j] < floor[i+1][j]) {
        check_neighbors(i+1,j,&floor,set);
    }
    if (j != 0) && (floor[i][j] < floor[i][j-1]) {
        check_neighbors(i,j-1,&floor,set);
    }
    if (j != floor[0].len()-1) && (floor[i][j] < floor[i][j+1]) {
        check_neighbors(i,j+1,&floor,set);
    }
}

fn day9_2() {
    let mut basins = Vec::new();
    let floor = include_str!("../../inputs/day9_input")
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    floor.iter()
        .enumerate()
        .for_each(|(i,r)| {
            r.iter()
                .enumerate()
                .for_each(|(j,c)| {
                    if ((i == 0) || (floor[i][j] < floor[i-1][j])) &&
                    ((i == floor.len()-1) || (floor[i][j] < floor[i+1][j])) &&
                    ((j == 0) || (floor[i][j] < floor[i][j-1])) &&
                    ((j == r.len()-1) || (floor[i][j] < floor[i][j+1])) {
                        let mut basin: HashSet<(usize, usize)> = HashSet::new();
                        check_neighbors(i,j,&floor, &mut basin);
                        basins.push(basin.len());
                    }
                })
        });
    basins.sort_unstable();
    println!("{:?}", basins.iter().rev().take(3).product::<usize>())
}

fn main() {
    let start = Instant::now();
    day9_2();
    let duration = start.elapsed();

    println!("Time elapsed is: {:?}", duration);
}