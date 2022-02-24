#![allow(dead_code)]
use advent::read_lines;
use std::fmt;

type PointType = u16;

struct OceanFloor {
    rows: Vec<Vec<u8>>
}

static mut DO_DIAGONAL: bool = false;

impl OceanFloor {
    fn new() -> OceanFloor {
        OceanFloor { rows: vec![vec![0; 1000]; 1000]}
    }

    fn add_horizontal_vent(&mut self, vent: &Vent) {
        let y: PointType = vent.start.y;
        let (min_x, max_x) = (vent.start.x.min(vent.stop.x),vent.start.x.max(vent.stop.x));
        for x in min_x..=max_x {
            self.rows[x as usize][y as usize] += 1;            
        }
    }

    fn add_vertical_vent(&mut self, vent: &Vent) {
        let x: PointType = vent.start.x;
        let (min_y, max_y) = (vent.start.y.min(vent.stop.y),vent.start.y.max(vent.stop.y));
        for y in min_y..=max_y {
            self.rows[x as usize][y as usize] += 1;            
        }
    }

    fn add_diagonal_vent(&mut self, vent: &Vent) {
        if vent.start.y <= vent.stop.y {
            let diff = vent.stop.y - vent.start.y;
            if vent.start.x <= vent.stop.x {
                for i in 0..=diff {
                    self.rows[(vent.start.x + i) as usize][(vent.start.y + i) as usize] += 1;
                }                  
            } else {
                for i in 0..=diff {
                    self.rows[(vent.start.x - i) as usize][(vent.start.y + i) as usize] += 1;
                }                  
            }  
        } else {
            let diff = vent.start.y - vent.stop.y;
            if vent.start.x <= vent.stop.x {
                for i in 0..=diff {
                    self.rows[(vent.start.x + i) as usize][(vent.start.y - i) as usize] += 1;
                }                  
            } else {
                for i in 0..=diff {
                    self.rows[(vent.start.x - i) as usize][(vent.start.y - i) as usize] += 1;
                }                  
            }
        }        
    }

    fn add_vent(&mut self, vent: Vent) {
        unsafe {
            if vent.start.y == vent.stop.y {
                self.add_horizontal_vent(&vent);
            } 
            else if vent.start.x == vent.stop.x {
                self.add_vertical_vent(&vent);
            }
            else if DO_DIAGONAL {
                self.add_diagonal_vent(&vent);
            }            
        }
    }

    fn calculate_overlap(&self) {
        let mut solution = 0;
        for row in &self.rows {
            solution += row.iter().filter(|val| **val > 1).count();
        }
        println!("{}", solution);
    }
}

impl fmt::Display for OceanFloor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.rows.len() {
            for row in &self.rows {
                write!(f, "{}", row[i])?;
            }
            write!(f, "\n")?;                
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Point {
    x: PointType,
    y: PointType,
}

impl Point {
    fn from(x: PointType, y: PointType) -> Point {
        Point {x,y}
    }
}

#[derive(Debug)]
struct Vent {
    start: Point,
    stop: Point,
}

impl Vent {
    fn from(start: Point, stop: Point) -> Vent {
        Vent {start, stop}
    }
}

fn day5_1() {
    let lines = read_lines("inputs/day5_input");
    let mut ocean_floor = OceanFloor::new();

    for line in lines {
        let line = line.unwrap();
        let line = line.split(" -> ").collect::<Vec<_>>();

        let vent = line.iter()
                        .map(|s| {
                            let s = s.split(',');
                            s.map(|p| p.parse::<PointType>().unwrap())
                                .collect::<Vec<_>>()
                        }).collect::<Vec<_>>();

        let vent = Vent::from(Point::from(vent[0][0],vent[0][1]), 
                               Point::from(vent[1][0],vent[1][1]));

        ocean_floor.add_vent(vent);
    }
    ocean_floor.calculate_overlap();    
}

fn day5_2() {
    unsafe {
        DO_DIAGONAL = true;
    }
    day5_1();    
}

fn main() {
    day5_2();
}