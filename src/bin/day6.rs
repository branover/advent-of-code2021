#![allow(dead_code)]
use advent::read_lines;
use std::{fmt, io::BufRead};
use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Debug)]
struct LanternFish {
    days_till_reproduction: u16,
}


fn day6_1() {
    let lines = read_lines("inputs/day6_input");
    let mut lantern_fishes = vec![];

    lines.for_each(|line| {
        let line = line.unwrap();
        line
            .split(',')
            .for_each(|fish| lantern_fishes.push(LanternFish{days_till_reproduction: fish.parse().unwrap()}));
    });

    for day in 0..80 {
        let mut new_fishes = vec![];
        lantern_fishes.iter_mut().for_each(|fish| {
            if fish.days_till_reproduction == 0 {
                fish.days_till_reproduction = 6;
                new_fishes.push(LanternFish{days_till_reproduction:8});
            } else {
                fish.days_till_reproduction -= 1;
            }
        });
        lantern_fishes.append(&mut new_fishes);
        
    }

    println!("{}", lantern_fishes.len());
   
}

fn day6_2() {
    let mut lines = read_lines("inputs/day6_input");
    let mut lantern_fishes = lines.next()
        .unwrap()
        .unwrap()
        .split(',')
        .fold([0; 9],|mut fish, n| {
            fish[n.parse::<usize>().unwrap()] += 1;
            fish
        });

    for _ in 0..256 {
        let mut new_fishes = [0; 9];
        new_fishes[8] += lantern_fishes[0];
        new_fishes[6] += lantern_fishes[0];

        for i in 1..lantern_fishes.len() {
            new_fishes[i-1] += lantern_fishes[i as usize];
        }
        lantern_fishes = new_fishes;     
    }

    println!("{:?}", lantern_fishes.iter().sum::<usize>());
}

fn main() {
    let start = Instant::now();
    day6_2();
    let duration = start.elapsed();

    println!("Time elapsed is: {:?}", duration);
}