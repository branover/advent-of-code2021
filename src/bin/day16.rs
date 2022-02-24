#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use advent::read_lines;
use std::fmt::Display;
use std::{fmt, io::BufRead};
use std::collections::{HashMap};
use std::time::{Duration, Instant};
use itertools::Itertools;
use std::ops::{BitAnd,BitOr};
use std::cmp::PartialOrd;
use std::mem::size_of;
use num::{Num,Integer,Unsigned,PrimInt};
use std::fmt::Binary;

type PacketVersion = usize;

fn u8_to_boolvec(num: u8, bool_vec: &mut Vec<bool>) {
    (0..8).for_each(|i|{
        bool_vec.push((num & (0b10000000 >> i)) > 0);        
    });
}

fn boolvec_to_uint<T>(vec: &[bool]) -> T
where
    T: PrimInt+Binary+BitAnd<Output = T>+BitOr<Output = T> + std::fmt::Display
{
    let mut ret: T = T::zero();
    let max = ((size_of::<T>()) * 8).min(vec.len()) - 1;
    for i in 0..=max {
        let mask = T::one().unsigned_shl(max as u32 - i as u32);
        if vec[i] == true {
            ret = ret | mask;
        }
    }
    ret
}

fn parse_literal(stream: &[bool]) -> (usize,PacketVersion,usize) {
    let mut literal = Vec::<bool>::new();
    let mut pos = 0;

    loop {
        let cont = stream[pos];
        pos += 1;
        
        let mut lit_slice = stream[pos..pos+4].to_vec();
        pos += 4;

        literal.append(&mut lit_slice);
        if !cont {break};
    }
    (boolvec_to_uint::<usize>(&literal), 0, pos)
}

fn parse_operator(stream: &[bool], type_id: u8) -> (usize,PacketVersion,usize) {
    let mut inner_version: PacketVersion = 0;
    let mut pos = 0;
    let sub_packet_len_size = match stream[pos] {
        true => 11,
        false => 15,
    };
    pos += 1;

    let sub_packet_len = boolvec_to_uint::<usize>(&stream[pos..pos+sub_packet_len_size]) as usize;
    pos += sub_packet_len_size;

    let mut inner_values: Vec<usize> = Vec::new();

    match sub_packet_len_size {
        11 => {
            for i in 0..sub_packet_len {
                let (value,new_inner_version,new_pos) = parse_packet(&stream[pos..]);
                pos += new_pos;
                inner_version += new_inner_version;
                inner_values.push(value as usize);
            }
        },
        15 => {
            let mut packet_len_read = 0;
            while packet_len_read < sub_packet_len {
                let (value,new_inner_version,new_pos) = parse_packet(&stream[pos..]);
                pos += new_pos;
                inner_version += new_inner_version;
                packet_len_read += new_pos;
                inner_values.push(value as usize);
            }
        }
        _ => panic!("Parsing error")
    };

    let value = match type_id {
        0 => inner_values.iter().sum(),
        1 => inner_values.iter().product(),
        2 => *inner_values.iter().min().unwrap(),
        3 => *inner_values.iter().max().unwrap(),
        5 => match inner_values[0] > inner_values[1] {
            true => 1,
            false => 0
        },
        6 => match inner_values[0] < inner_values[1] {
            true => 1,
            false => 0
        },
        7 => match inner_values[0] == inner_values[1] {
            true => 1,
            false => 0
        },
        _ => panic!("Invalid typeid")
    };
    (value, inner_version, pos)
}


fn parse_packet(stream: &[bool]) -> (usize,PacketVersion,usize) {
    let mut pos = 0;
    let version: PacketVersion = boolvec_to_uint::<u8>(&stream[pos..pos+3]) as usize;
    pos += 3;

    let type_id = boolvec_to_uint::<u8>(&stream[pos..pos+3]);
    pos += 3;

    let (literal,inner_version,inc) = match type_id {
        4 => parse_literal(&stream[pos..]),      
        _ => parse_operator(&stream[pos..],type_id)
    };
    pos += inc;

    (literal,version + inner_version,pos)
}

fn day16_1() {
    let stream = hex::decode(include_str!("../../inputs/day16_input")).unwrap();
    let mut boolvec = Vec::<bool>::new();
    stream.iter().for_each(|b| {
        u8_to_boolvec(*b, &mut boolvec);
    });
    let (_,version_sum,_) = parse_packet(&boolvec[..]);
    println!("Solution: {}", version_sum);
}


fn day16_2() {
    let stream = hex::decode(include_str!("../../inputs/day16_input")).unwrap();
    let mut boolvec = Vec::<bool>::new();
    stream.iter().for_each(|b| {
        u8_to_boolvec(*b, &mut boolvec);
    });
    let (value,_,_) = parse_packet(&boolvec[..]);
    println!("Solution: {}", value);
}

fn main() {
    let start = Instant::now();
    day16_2();
    let duration = start.elapsed();

    println!("Time elapsed is: {:?}", duration);
}