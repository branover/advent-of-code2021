#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![feature(let_else)]
#![feature(slice_group_by)]
#![feature(drain_filter)]
use advent::read_lines;
use std::fmt::Display;
use std::{fmt, io::BufRead};
use std::collections::{HashMap};
use std::time::{Duration, Instant};
use itertools::Itertools;


#[derive(Clone, Debug)]
pub struct Scanner {
    vision: Vec<(isize, isize, isize)>,
}

pub fn input_generator(input: &str) -> Vec<Scanner> {
    input
        .split("\n\n")
        .map(|scannerlines| {
            let mut lines = scannerlines.lines();
            let _ = lines.next().expect("At least the scanner description line");
            Scanner {
                vision: lines.map(|line| {
                    let (x, line) = line.split_once(',').expect("not valid coord");
                    let (y, z) = line.split_once(',').expect("not valid coord");
                    let x = x.parse().expect("coord x not num");
                    let y = y.parse().expect("coord y not num");
                    let z = z.parse().expect("coord z not num");
                    (x, y, z)
                }).collect(),
            }
        })
        .collect()
}
pub fn solve_part1(input: &[Scanner]) -> Option<usize> {
    let (_, mut beacons) = solve(input);
    beacons.sort_unstable();
    beacons.dedup();
    Some(beacons.len())
}
pub fn solve_part2(input: &[Scanner]) -> Option<usize> {
    let (scanners, _) = solve(input);
    scanners.iter().map(|lhs| scanners.iter().map(move |rhs| (lhs, rhs))).flatten()
        .map(|((_, (lx, ly, lz)), (_, (rx, ry, rz)))| {
            ((lx - rx).abs() + (ly - ry).abs() + (lz - rz).abs()) as usize
        })
        .max()
}


fn normalise_coord((mut x, mut y, mut z): (isize, isize, isize)) -> (isize, isize, isize){
    let valsort = |lhs: &mut isize, rhs: &mut isize| {
        if lhs.abs() > rhs.abs() {
            let (olhs, orhs) = (*lhs, *rhs);
            if *rhs <= 0 {
                *rhs = olhs;
                *lhs = -orhs;
            } else {
                *rhs = -olhs;
                *lhs = orhs;
            }
        }
    };
    valsort(&mut x, &mut z);
    valsort(&mut x, &mut y);
    valsort(&mut y, &mut z);
    if x < 0 {
        x = -x;
        y = -y;
    }
    if y < 0 {
        y = -y;
        z = -z;
    }
    (x, y, z)
}
fn find_valid_transformer(from: (isize, isize, isize), to: (isize, isize, isize)) -> (usize, &'static dyn Fn((isize, isize, isize)) -> (isize, isize, isize)) {
    let transformation: [&dyn Fn((isize, isize, isize)) -> (isize, isize, isize); 24] = [
        &|(x, y, z)| { let (x, y, z) = ( x, y, z ); (x, y, z) }.into(), // centre
        &|(x, y, z)| { let (x, y, z) = ( x, y, z ); (-y, x, z) }.into(),
        &|(x, y, z)| { let (x, y, z) = ( x, y, z ); (y, -x, z) }.into(),
        &|(x, y, z)| { let (x, y, z) = ( x, y, z ); (-x, -y, z) }.into(),

        &|(x, y, z)| { let (x, y, z) = ( x, -z, y ); (x, y, z) }.into(), // left
        &|(x, y, z)| { let (x, y, z) = ( x, -z, y ); (-y, x, z) }.into(),
        &|(x, y, z)| { let (x, y, z) = ( x, -z, y ); (y, -x, z) }.into(),
        &|(x, y, z)| { let (x, y, z) = ( x, -z, y ); (-x, -y, z) }.into(),

        &|(x, y, z)| { let (x, y, z) = ( x, z, -y ); (x, y, z) }.into(), // right
        &|(x, y, z)| { let (x, y, z) = ( x, z, -y ); (-y, x, z) }.into(),
        &|(x, y, z)| { let (x, y, z) = ( x, z, -y ); (y, -x, z) }.into(),
        &|(x, y, z)| { let (x, y, z) = ( x, z, -y ); (-x, -y, z) }.into(),

        &|(x, y, z)| { let (x, y, z) = ( x, -y, -z ); (x, y, z) }.into(), // 180
        &|(x, y, z)| { let (x, y, z) = ( x, -y, -z ); (-y, x, z) }.into(),
        &|(x, y, z)| { let (x, y, z) = ( x, -y, -z ); (y, -x, z) }.into(),
        &|(x, y, z)| { let (x, y, z) = ( x, -y, -z ); (-x, -y, z) }.into(),

        &|(x, y, z)| { let (x, y, z) = ( z, y, -x ); (x, y, z) }.into(), // up
        &|(x, y, z)| { let (x, y, z) = ( z, y, -x ); (-y, x, z) }.into(),
        &|(x, y, z)| { let (x, y, z) = ( z, y, -x ); (y, -x, z) }.into(),
        &|(x, y, z)| { let (x, y, z) = ( z, y, -x ); (-x, -y, z) }.into(),

        &|(x, y, z)| { let (x, y, z) = ( -z, y, x ); (x, y, z) }.into(), // down
        &|(x, y, z)| { let (x, y, z) = ( -z, y, x ); (-y, x, z) }.into(),
        &|(x, y, z)| { let (x, y, z) = ( -z, y, x ); (y, -x, z) }.into(),
        &|(x, y, z)| { let (x, y, z) = ( -z, y, x ); (-x, -y, z) }.into(),

    ];

    for (idx, tran) in transformation.iter().copied().enumerate() {
        if tran(from) == to {
            return (idx, tran);
        }
    }
    unreachable!()
}

fn solve(input: &[Scanner]) -> (Vec<(usize, (isize, isize, isize))>, Vec<(isize, isize, isize)>) {
    let mut norms: Vec<(usize, Vec<((isize, isize, isize), (isize, isize, isize), (isize, isize, isize))>)> = input.iter().enumerate()
        .map(|(sidx, s)| {
            let beacons = s.vision.as_slice();
            let mut norm: Vec<_> = beacons.iter().map(|lhs| beacons.iter().map(move |rhs| (lhs, rhs))).flatten()
                .filter(|(lhs, rhs)| lhs != rhs)
                .map(|(&(lx, ly, lz), &(rx, ry, rz))| {
                    let norm = normalise_coord((lx - rx, ly - ry, lz - rz));
                    (norm, (lx, ly, lz), (rx, ry, rz))
                })
                .collect();
            norm.sort_unstable();
            (sidx, norm)
        })
        .collect();
    let (psidx, mut active_norms) = norms.remove(0);
    let mut full_norms = active_norms.clone();
    let mut scanner_loc = vec![(0, (0, 0, 0))];
    let mut know_beacon = input[psidx].vision.clone();

    
    let mut candidatecount = vec![];
    while !norms.is_empty() {
        // rebuild the list of norms from the full set
        active_norms.clear();
        active_norms.extend(full_norms.iter().copied());
        active_norms.sort_unstable();
        norms
            .drain_filter(|(nidx, n)| {
                candidatecount.clear();
                let mut lhsiter = active_norms.group_by(|lhs, rhs| lhs.0 == rhs.0).peekable();
                let mut rhsiter = n.group_by(|lhs, rhs| lhs.0 == rhs.0).peekable();
                while lhsiter.peek().is_some() && rhsiter.peek().is_some() {
                    let (Some(lhs), Some(rhs)) = (lhsiter.peek(), rhsiter.peek()) else { unreachable!() };
                    match lhs[0].0.cmp(&rhs[0].0) {
                        std::cmp::Ordering::Equal => {
                            for (lhs, rhs) in lhs.iter().map(|lhs| rhs.iter().map(move |rhs| (lhs, rhs))).flatten() {
                                let (lx, ly, lz) = (lhs.2.0 - lhs.1.0, lhs.2.1 - lhs.1.1, lhs.2.2 - lhs.1.2);
                                let (rx, ry, rz) = (rhs.2.0 - rhs.1.0, rhs.2.1 - rhs.1.1, rhs.2.2 - rhs.1.2);
                                let (tranid, tran) = find_valid_transformer((rx, ry, rz), (lx, ly, lz));
                                let (lsx, lsy, lsz) = lhs.1;
                                let (rsx, rsy, rsz) = tran(rhs.1);
                                let offset = (lsx - rsx, lsy - rsy, lsz - rsz);
                                candidatecount.push((offset, tranid, tran));
                            }
                            lhsiter.next();
                            rhsiter.next();
                        },
                        std::cmp::Ordering::Less => {
                            lhsiter.next();
                        },
                        std::cmp::Ordering::Greater => {
                            rhsiter.next();
                        },
                    }
                }
                candidatecount.sort_unstable_by_key(|(offset, tranid, _tran)| (*offset, *tranid));
                let transinfo = candidatecount.group_by(|lhs, rhs| (lhs.0, lhs.1) == (rhs.0, rhs.1))
                    .filter(|candidate| candidate.len() >= 6)
                    .max_by_key(|candidate| candidate.len());
                let Some(candidate) = transinfo else { return false };
                let ((ox, oy, oz), _tranid, tran) = candidate[0];
                // We determined the transformation, update list of normalindex, the know beacon list and known scanner location list
                full_norms.extend(n.iter().map(|(norm, lhs, rhs)| {
                    let (tx, ty, tz) = tran(*lhs);
                    let lhs = (tx + ox, ty + oy, tz + oz);
                    let (tx, ty, tz) = tran(*rhs);
                    let rhs = (tx + ox, ty + oy, tz + oz);
                    (*norm, lhs, rhs)
                }));
                
                know_beacon.extend(input[*nidx].vision.iter().map(|c| {
                    let (tx, ty, tz) = tran(*c);
                    (tx + ox, ty + oy, tz + oz)
                }));
                scanner_loc.push((*nidx, (ox, oy, oz)));
                true
            })
            .for_each(|_| ());
    }
    (scanner_loc, know_beacon)
}

fn day19_1() {
    let solution = include_str!("../../inputs/day19_input");
    let solution = input_generator(&solution);
    let solution = solve_part1(&solution).unwrap();
        
    println!("{}",solution);
}


fn day19_2() {
    let solution = include_str!("../../inputs/day19_input");
    let solution = input_generator(&solution);
    let solution = solve_part2(&solution).unwrap();
        
    println!("{}",solution);
}

fn main() {
    let start = Instant::now();
    day19_2();
    let duration = start.elapsed();

    println!("Time elapsed is: {:?}", duration);
}