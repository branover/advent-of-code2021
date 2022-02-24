#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use advent::read_lines;
use std::{fmt, io::BufRead};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use itertools::Itertools;

fn day8_1() {
    let lines = read_lines("inputs/day8_input");
    let solution: usize = lines
        .map(|line| line.unwrap())
        .map(|line| line.split_once('|').unwrap().1.to_string())
        .collect::<Vec<_>>()
        .iter()
        .map(|s| {
            s.split_whitespace()
                .filter(|word| {
                    matches!(word.len(), 2 | 3 | 4 | 7)
                })
                .count()
        })
        .sum();
    println!("{}", solution);
}

fn process_line(sequence: &str, digits: &str) -> usize {
    let mut letter_map: HashMap<char,char> = HashMap::new();
    let mut sequence = sequence.split_whitespace()
        .collect::<Vec<_>>();
    sequence.sort_by(|a,b| a.len().cmp(&b.len()));

    // Find a
    letter_map.insert('a',
        sequence[1].chars()
            .filter(|c| !sequence[0].contains(*c))
            .next()
            .unwrap()
    );
    // Find f
    letter_map.insert('f', 
        sequence[0].chars()
            .filter(|c| {
                sequence[6..=8].iter().all(|s| s.contains(*c))
            })
            .next()
            .unwrap()
    );
    // Find c
    letter_map.insert('c', 
        sequence[0].chars().filter(|c| letter_map[&'f'] != *c).next().unwrap()
    );
    // Find d
    letter_map.insert('d', 
    sequence[2].chars()
        .filter(|c| {
            (sequence.iter().filter(|s| s.contains(*c)).count() == 7) &&
            // sequence[2].contains(*c) &&
            (letter_map[&'f'] != *c)
        })
        .next()
        .unwrap()
    );
    // Find b
    letter_map.insert('b', 
    sequence[2].chars()
        .filter(|c| {
            letter_map.values().all(|v| v != c)
        })
        .next()
        .unwrap()
    );
    // Find g
    letter_map.insert('g', 
    sequence[6..=8].iter()
        .filter(|s| {
            sequence[2].chars().all(|c| s.contains(c))
        })
        .next()
        .unwrap()
        .chars()
        .filter(|c| !letter_map.values().any(|v| c == v))
        .next()
        .unwrap()
    );
    // Find e
    letter_map.insert('e', 
    sequence[9].chars()
        .filter(|c| {
            letter_map.values().all(|v| v != c)
        })
        .next()
        .unwrap()
    );

    let digits: Vec<String> = digits.split_whitespace()
        .map(|s| {
            s.chars()
                .map(|c| {
                    letter_map.iter()
                        .filter(|(k,v)| **v == c)
                        .map(|(k,v)| k)
                        .collect::<String>()
                })
                .collect()
        })
        .collect();

    let digits_map: HashMap<&str, char> = HashMap::from([
        ("abcefg",'0'),
        ("cf",'1'),
        ("acdeg",'2'),
        ("acdfg",'3'),
        ("bcdf",'4'),
        ("abdfg",'5'),
        ("abdefg",'6'),
        ("acf",'7'),
        ("abcdefg",'8'),
        ("abcdfg",'9'),
    ]);

    let answer = digits.iter()
        .map(|s| {
            let s: String = s.chars().sorted().collect();
            digits_map.get(&s.as_str()).unwrap().clone()
        })
            .collect::<String>()
            .parse::<usize>()
            .unwrap();

    answer
}

fn day8_2() {
    let lines = read_lines("inputs/day8_input");
    let solution: usize = lines
        .map(|line| line.unwrap())
        .map(|line| {
            let mut line = line.split('|');
            process_line(line.next().unwrap(),
                line.next().unwrap())
        })
        .collect::<Vec<_>>()
        .iter()
        .sum();
    println!("{}", solution);
}

fn main() {
    let start = Instant::now();
    day8_1();
    let duration = start.elapsed();

    println!("Time elapsed is: {:?}", duration);
}