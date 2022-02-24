#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use advent::read_lines;
use std::fmt::Display;
use std::{fmt, io::BufRead};
use std::collections::{HashMap};
use std::time::{Duration, Instant};
use itertools::Itertools;
use std::ops::RangeInclusive;
use std::num;

fn run_sim(x_start: i32,
    y_start: i32,
    x_target: &RangeInclusive<i32>,
    y_target: &RangeInclusive<i32>)
    -> (bool,i32) {
    let (mut x_veloc, mut y_veloc) = (x_start, y_start);
    let (mut x,mut y) = (0,0);
    let mut max_y = 0;
    let x_upper = x_target.clone().last().unwrap();
    let y_lower = y_target.clone().next().unwrap();
    while y > y_lower && x < x_upper {
        x += x_veloc;
        y += y_veloc;
        if y > max_y {
            max_y = y;
        }
        x_veloc = match x_veloc {
            x if x > 0 => x_veloc - 1,
            x if x < 0 => x_veloc + 1,
            _ => x_veloc
        };
        y_veloc -= 1;
        if y_target.contains(&y) && x_target.contains(&x) {
            return (true,max_y);
        }
    }
    (false,0)
}

fn day17_1() {
    let (x_min,x_max) = (57,116);
    let (y_min,y_max) = (-198,-148);
    // let (x_min,x_max) = (20,30);
    // let (y_min,y_max) = (-10,-5);

    let x_range = x_min..=x_max;
    let y_range = y_min..=y_max;

    let x_min_veloc = (x_min as f32 * 2.0).sqrt() as i32 + 1;

    let solution = (y_min..-y_min).map(|y| {
        (x_min_veloc..=x_max).map(|x| {
            run_sim(x,y,&x_range,&y_range).1            
        }).max().unwrap()
    }).max().unwrap();
        
    println!("{}",solution);
}


fn day17_2() {
    let (x_min,x_max) = (57,116);
    let (y_min,y_max) = (-198,-148);
    // let (x_min,x_max) = (20,30);
    // let (y_min,y_max) = (-10,-5);

    let x_range = x_min..=x_max;
    let y_range = y_min..=y_max;

    let x_min_veloc = (x_min as f32 * 2.0).sqrt() as i32 + 1;

    let solution: usize = (y_min..-y_min).map(|y| {
        (x_min_veloc..=x_max).map(|x| {
            run_sim(x,y,&x_range,&y_range)            
        }).filter(|(hit_target,_)| *hit_target).count()
    }).sum::<usize>();
        
    println!("{}",solution);
}

fn main() {
    let start = Instant::now();
    day17_2();
    let duration = start.elapsed();

    println!("Time elapsed is: {:?}", duration);
}