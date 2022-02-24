#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use advent::read_lines;
use std::fmt::Display;
use std::{fmt, io::BufRead};
use std::collections::{HashMap};
use std::time::{Duration, Instant};
use itertools::Itertools;
use std::ops::Add;

use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::map,
    sequence::{delimited, separated_pair},
    IResult,
};

#[derive(Debug, Clone)]
enum Number {
    Regular(u64),
    Pair(Box<Number>, Box<Number>),
}

impl Add for Number {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Pair(Box::new(self), Box::new(rhs))
    }
}

impl Number {
    fn reduce(self) -> Self {
        let mut number = self;
        loop {
            let (next_number, res) = number.explode(0);
            number = next_number;
            if res.is_some() {
                continue;
            };
            let (next_number, res) = number.split();
            number = next_number;
            if !res {
                break;
            }
        }
        number
    }

    fn explode(self, depth: usize) -> (Self, Option<(Option<u64>, Option<u64>)>) {
        match self {
            Self::Regular(_) => (self, None),
            Self::Pair(l, r) => match (*l, *r) {
                (Self::Regular(nl), Self::Regular(nr)) if depth >= 4 => {
                    (Self::Regular(0), Some((Some(nl), Some(nr))))
                }
                (l, r) => match l.explode(depth + 1) {
                    (l_reduced, Some((explode_left, explode_right))) => {
                        let r_added = if let Some(explode_right) = explode_right {
                            r.add_to_leftmost(explode_right)
                        } else {
                            r
                        };
                        (
                            Self::Pair(Box::new(l_reduced), Box::new(r_added)),
                            Some((explode_left, None)),
                        )
                    }
                    (l_reduced, None) => match r.explode(depth + 1) {
                        (r_reduced, Some((explode_left, explode_right))) => {
                            let l_added = if let Some(explode_left) = explode_left {
                                l_reduced.add_to_rightmost(explode_left)
                            } else {
                                l_reduced
                            };
                            (
                                Self::Pair(Box::new(l_added), Box::new(r_reduced)),
                                Some((None, explode_right)),
                            )
                        }
                        (r_reduced, None) => {
                            (Self::Pair(Box::new(l_reduced), Box::new(r_reduced)), None)
                        }
                    },
                },
            },
        }
    }

    fn split(self) -> (Self, bool) {
        match self {
            Self::Regular(n) if n >= 10 => (
                Self::Pair(
                    Box::new(Self::Regular(n / 2)),
                    Box::new(Self::Regular(if n % 2 == 0 { n / 2 } else { n / 2 + 1 })),
                ),
                true,
            ),
            Self::Regular(_) => (self, false),
            Self::Pair(l, r) => {
                let (l_split, l_was_split) = l.split();
                if l_was_split {
                    (Self::Pair(Box::new(l_split), r), true)
                } else {
                    let (r_split, r_was_split) = r.split();
                    (
                        Self::Pair(Box::new(l_split), Box::new(r_split)),
                        r_was_split,
                    )
                }
            }
        }
    }

    fn add_to_leftmost(self, val: u64) -> Self {
        match self {
            Number::Regular(n) => Number::Regular(n + val),
            Number::Pair(l, r) => Number::Pair(Box::new(l.add_to_leftmost(val)), r),
        }
    }

    fn add_to_rightmost(self, val: u64) -> Self {
        match self {
            Number::Regular(n) => Number::Regular(n + val),
            Number::Pair(l, r) => Number::Pair(l, Box::new(r.add_to_rightmost(val))),
        }
    }

    fn magnitude(&self) -> u64 {
        match self {
            Self::Regular(n) => *n,
            Self::Pair(l, r) => 3 * l.magnitude() + 2 * r.magnitude(),
        }
    }
}

fn parse_number(input: &str) -> IResult<&str, Number> {
    alt((
        map(nom::character::complete::u64, Number::Regular),
        map(
            delimited(
                tag("["),
                separated_pair(parse_number, tag(","), parse_number),
                tag("]"),
            ),
            |(l, r)| Number::Pair(Box::new(l), Box::new(r)),
        ),
    ))(input)
}

fn day18_1() {
    let input: Vec<_> = include_str!("../../inputs/day18_input")
        .lines()
        .map(|l| parse_number(&l).unwrap().1)
        .collect();
    
    let solution = {
        let sum = input
            .clone()
            .into_iter()
            .reduce(|l, r| (l + r.clone()).reduce())
            .unwrap();
        sum.magnitude()
    };    
        
    println!("{}",solution);
}


fn day18_2() {
    let input: Vec<_> = include_str!("../../inputs/day18_input")
        .lines()
        .map(|l| parse_number(&l).unwrap().1)
        .collect();
    
    let solution = {
        input
            .into_iter()
            .permutations(2)
            .map(|permutation| {
                permutation
                    .into_iter()
                    .reduce(|l, r| (l + r).reduce())
                    .unwrap()
                    .magnitude()
            })
            .max()
            .unwrap()
    };
        
    println!("{}",solution);
}

fn main() {
    let start = Instant::now();
    day18_2();
    let duration = start.elapsed();

    println!("Time elapsed is: {:?}", duration);
}