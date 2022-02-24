#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use advent::read_lines;
use std::fmt::Display;
use std::{fmt, io::BufRead};
use std::collections::{HashMap};
use std::time::{Duration, Instant};
use itertools::Itertools;

fn char_windows<'a>(src: &'a str, win_size: usize) -> impl Iterator<Item = &'a str> {
    src.char_indices()
        .flat_map(move |(from, _)| {
            src[from ..].char_indices()
                .skip(win_size - 1)
                .next()
                .map(|(to, c)| {
                    &src[from .. from + to + c.len_utf8()]
                })
    })
}

fn day14_1() {
    let (initial,template) = include_str!("../../inputs/day14_input").split_once("\n\n").unwrap();
    let template = template.lines()
        .map(|l| l.split_once(" -> ").unwrap())
        .map(|(s,c)| (s,c.chars().next().unwrap()))
        .collect::<HashMap<&str,char>>();
    let mut letters_count: HashMap<char,u32> = initial.chars()
        .fold(HashMap::new(), |mut map, c| {
            *map.entry(c).or_insert(0) += 1;
            map
        });
    let mut pairs_count: HashMap<&str,u32> = template.keys()
        .fold(HashMap::new(), |mut map, key| {
            map.insert(key, 0);
            map
        });
    char_windows(initial, 2).for_each(|c| {
        *pairs_count.get_mut(c).unwrap() += 1;
    });
    for i in 0..10 {
        let mut new_pairs = pairs_count.clone();
        pairs_count.iter().filter(|(pair,count)| **count > 0).for_each(|(pair,count)| {
            let new_char = template.get(pair).unwrap();
            let pair1 = [pair.chars().next().unwrap(),*new_char].iter().collect::<String>();
            let pair2 = [*new_char,pair.chars().nth(1).unwrap()].iter().collect::<String>();
            *new_pairs.get_mut(pair).unwrap() -= count;
            *new_pairs.get_mut(pair1.as_str()).unwrap() += count;
            *new_pairs.get_mut(pair2.as_str()).unwrap() += count;
            *letters_count.entry(*new_char).or_insert(0) += count;
        });
        pairs_count = new_pairs;
    }
    println!("{}",letters_count.values().max().unwrap() - letters_count.values().min().unwrap());
}


fn day14_2() {
    let (initial,template) = include_str!("../../inputs/day14_input").split_once("\n\n").unwrap();
    let template = template.lines()
        .map(|l| l.split_once(" -> ").unwrap())
        .map(|(s,c)| (s,c.chars().next().unwrap()))
        .collect::<HashMap<&str,char>>();
    let mut letters_count: HashMap<char,u64> = initial.chars()
        .fold(HashMap::new(), |mut map, c| {
            *map.entry(c).or_insert(0) += 1;
            map
        });
    let mut pairs_count: HashMap<&str,u64> = template.keys()
        .fold(HashMap::new(), |mut map, key| {
            map.insert(key, 0);
            map
        });
    char_windows(initial, 2).for_each(|c| {
        *pairs_count.get_mut(c).unwrap() += 1;
    });
    for i in 0..40 {
        let mut new_pairs = pairs_count.clone();
        pairs_count.iter().filter(|(pair,count)| **count > 0).for_each(|(pair,count)| {
            let new_char = template.get(pair).unwrap();
            let pair1 = [pair.chars().next().unwrap(),*new_char].iter().collect::<String>();
            let pair2 = [*new_char,pair.chars().nth(1).unwrap()].iter().collect::<String>();
            *new_pairs.get_mut(pair).unwrap() -= count;
            *new_pairs.get_mut(pair1.as_str()).unwrap() += count;
            *new_pairs.get_mut(pair2.as_str()).unwrap() += count;
            *letters_count.entry(*new_char).or_insert(0) += count;
        });
        pairs_count = new_pairs;
    }

    println!("{}",letters_count.values().max().unwrap() - letters_count.values().min().unwrap());
}

fn main() {
    let start = Instant::now();
    day14_2();
    let duration = start.elapsed();

    println!("Time elapsed is: {:?}", duration);
}