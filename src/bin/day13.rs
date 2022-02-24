#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use advent::read_lines;
use std::fmt::Display;
use std::{fmt, io::BufRead};
use std::collections::{HashMap};
use std::time::{Duration, Instant};
use itertools::Itertools;

fn fold_point(point: i16, line: i16) -> i16 {
    if point < line {
        point
    } 
    else {
        point - 2*(point - line)
    }
}

fn draw_points(points: &Vec<(i16,i16)>) {
    let max_x = *points.iter().map(|(x,y)| x).max().unwrap() as usize;
    let max_y = *points.iter().map(|(x,y)| y).max().unwrap() as usize;
    let mut grid = vec![vec!['.'; max_y+1]; max_x+1];
    points.iter().for_each(|(x,y)| {grid[*x as usize][*y as usize] = '#';});
    for y in 0..grid[0].len() {
        for x in 0..grid.len() {
            print!("{}", grid[x][y]);
        }
        print!("\n");
    }
}

fn day13_1() {
    let (points,folds) = include_str!("../../inputs/day13_input").split_once("\n\n").unwrap();
    let mut points = points.lines()
        .map(|l| {
            let mut l = l.split(',')
                .map(|s| s.parse::<i16>().unwrap());
            (l.next().unwrap(),l.next().unwrap())    
        }).collect::<Vec<_>>();
    let folds = folds.lines()
        .map(|l| {
            let l = l.split(' ').nth(2).unwrap();
            let (axis,line) = l.split_once('=').unwrap();
            let line = line.parse::<i16>().unwrap();
            (axis,line)
        }).collect::<Vec<_>>();
    folds[0..1].iter().for_each(|(axis,line)| {
        points = points.iter().map(|(x,y)| {
            match *axis {
                "y" => (*x,fold_point(*y, *line)),
                "x" => (fold_point(*x, *line),*y),
                _ => panic!("Unknown axis")
            }
        }).collect::<Vec<_>>();
    });
    points.sort_unstable();
    points.dedup();    
    println!("{:?}",points.len());
}


fn day13_2() {
    let (points,folds) = include_str!("../../inputs/day13_input").split_once("\n\n").unwrap();
    let mut points = points.lines()
        .map(|l| {
            let mut l = l.split(',')
                .map(|s| s.parse::<i16>().unwrap());
            (l.next().unwrap(),l.next().unwrap())    
        }).collect::<Vec<_>>();
    let folds = folds.lines()
        .map(|l| {
            let l = l.split(' ').nth(2).unwrap();
            let (axis,line) = l.split_once('=').unwrap();
            let line = line.parse::<i16>().unwrap();
            (axis,line)
        }).collect::<Vec<_>>();
    folds.iter().for_each(|(axis,line)| {
        points = points.iter().map(|(x,y)| {
            match *axis {
                "y" => (*x,fold_point(*y, *line)),
                "x" => (fold_point(*x, *line),*y),
                _ => panic!("Unknown axis")
            }
        }).collect::<Vec<_>>();
    });
    points.sort_unstable();
    points.dedup();    
    draw_points(&points);
}

fn main() {
    let start = Instant::now();
    day13_2();
    let duration = start.elapsed();

    println!("Time elapsed is: {:?}", duration);
}