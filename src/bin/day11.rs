#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use advent::read_lines;
use std::fmt::Display;
use std::{fmt, io::BufRead};
use std::collections::{HashMap};
use std::time::{Duration, Instant};
use itertools::Itertools;

const NEXT: [(isize, isize); 8] = [
    (-1, -1), (0, -1), (1, -1),
    (-1, 0),           (1, 0),
    (-1, 1),  (0, 1),  (1, 1),
    ];

struct Octopuses {
    table: Vec<Vec<Octopus>>
}

struct Octopus {
    energy: u8,
    already_flashed: bool,
}

impl Display for Octopuses {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.table.iter().for_each(|r| {
            r.iter().for_each(|o| {write!(f, "{}", o.energy).unwrap();});
            write!(f, "\n").unwrap();
        });
        Ok(())
    }
}

fn flash_neighbors(x: usize, y: usize, table: &mut Vec<Vec<Octopus>>) -> i32 {
    let mut octopus = &mut table[x][y];
    octopus.energy += 1;
    if octopus.energy > 9 && !octopus.already_flashed {
        octopus.already_flashed = true;
        NEXT.iter().map(|(xx,yy)|((x as isize + xx) as usize, (y as isize + yy) as usize))
            .fold(1, |sum, (xxx, yyy)| {
                match table.get(yyy).and_then(|l| l.get(xxx)) {
                    Some(o) => sum + flash_neighbors(xxx,yyy, table),
                    None => sum,
                }
            })        
    }
    else {
        0
    }
}

fn day11_1() {
    let octopuses = include_bytes!("../../inputs/day11_input")
        .split(|&b| b == b'\n')
        .map(|l| l.iter().map(|&c| {
            Octopus {energy: (c as char).to_digit(10).unwrap() as u8, already_flashed: false}
        }).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut octopuses = Octopuses {table: octopuses};
    let mut flashes = 0;
    for i in 0..100 {
        let mut table = &mut octopuses.table;
        table.iter_mut().for_each(|row| row.iter_mut().for_each(|o| o.energy +=1));
        for y in 0..table.len() {
            for x in 0..table[0].len() {
                if table[x][y].energy > 9 {
                    flashes += flash_neighbors(x, y, &mut table);
                }
            }
        }
        table.iter_mut().for_each(|row| row.iter_mut().filter(|o| o.energy > 9).for_each(|o| {*o = Octopus{energy: 0, already_flashed:false}}));
    }
    println!("{}",flashes);
}


fn day11_2() {
    let octopuses = include_bytes!("../../inputs/day11_input")
        .split(|&b| b == b'\n')
        .map(|l| l.iter().map(|&c| {
            Octopus {energy: (c as char).to_digit(10).unwrap() as u8, already_flashed: false}
        }).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut octopuses = Octopuses {table: octopuses};
    let mut flashes = 0;
    for i in 1..1000 {
        let mut table = &mut octopuses.table;
        table.iter_mut().for_each(|row| row.iter_mut().for_each(|o| o.energy +=1));
        for y in 0..table.len() {
            for x in 0..table[0].len() {
                if table[x][y].energy > 9 {
                    flashes += flash_neighbors(x, y, &mut table);
                }
            }
        }
        let flashed: usize = table.iter().map(|row| row.iter().filter(|o| o.already_flashed).count()).sum();
        if flashed == (table[0].len() * table.len()) {
            println!("{}", i);
            break;
        }
        table.iter_mut().for_each(|row| row.iter_mut().filter(|o| o.energy > 9).for_each(|o| {*o = Octopus{energy: 0, already_flashed:false}}));
    }
}

fn main() {
    let start = Instant::now();
    day11_2();
    let duration = start.elapsed();

    println!("Time elapsed is: {:?}", duration);
}