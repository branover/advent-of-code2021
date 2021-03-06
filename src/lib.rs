use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;



pub fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<File>>
where P: AsRef<Path>, {
    let file = File::open(filename).expect("Couldn't open file");
    io::BufReader::new(file).lines()
}