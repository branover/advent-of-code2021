#![allow(dead_code)]
use advent::read_lines;

fn day3_1() {
    let lines = read_lines("inputs/day3_input");
    let mut positions: Vec<(u32, u32)> = vec![(0,0); 12];

    for line in lines {
        let line = line.unwrap();
        for (i,c) in line.chars().enumerate() {
            match c {
                '1' => positions[i].1 += 1,
                '0' => positions[i].0 += 1,
                _ => (),
            }
        }
    }

    let (gamma, epsilon): (String,String) = positions.iter().map(|tup| {
        if tup.1 > tup.0 {('0','1')}
        else {('1','0')}
    }).unzip();

    let gamma = usize::from_str_radix(&gamma, 2).unwrap();
    let epsilon = usize::from_str_radix(&epsilon, 2).unwrap();

    println!("{}", gamma * epsilon);
}

fn day3_2() {
    fn greatest_in_pos(lines: &Vec<String>, i: usize) -> (char,char) {
        let mut positions: Vec<(u32, u32)> = vec![(0,0); 12];

        for line in lines {
            for (i,c) in line.chars().enumerate() {
                match c {
                    '0' => positions[i].0 += 1,
                    '1' => positions[i].1 += 1,
                    _ => (),
                }
            }
        }

        if positions[i].1 >= positions[i].0 {('1','0')}
        else {('0','1')}    
    }

    fn eval_bit_criteria(greatest: bool) -> String {
        let lines = read_lines("inputs/day3_input");
        let mut current_set: Vec<String> = lines.map(|x| x.unwrap()).collect();

        fn filter_index(current_set: Vec<String>, index: usize, take_val: char) -> Vec<String> {
            let mut result = Vec::new();

            for entry in current_set {
                if entry.chars().nth(index).unwrap() == take_val {
                    result.push(entry)
                }
            }
            result
        }

        for i in 0..12 {
            let (g,l) = greatest_in_pos(&current_set, i);
            if greatest {
                current_set = filter_index(current_set, i, g);
            } else {
                current_set = filter_index(current_set, i, l);
            }
            if current_set.len() == 1 {
                return String::from(&current_set[0])
            }
        }
        String::from(&current_set[0])
    }

    let oxygen_generator = eval_bit_criteria(true);
    let co2_scrubber = eval_bit_criteria(false);
    
    let oxygen_generator = usize::from_str_radix(&oxygen_generator, 2).unwrap();
    let co2_scrubber = usize::from_str_radix(&co2_scrubber, 2).unwrap();

    println!("{}", oxygen_generator * co2_scrubber);
}

fn main() {
    day3_2();
}