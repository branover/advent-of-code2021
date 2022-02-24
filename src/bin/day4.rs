#![allow(dead_code)]
use advent::read_lines;

#[derive(Debug)]
struct Square {
    num: u8,
    picked: bool,
}

impl Square {
    fn from(num: u8) -> Square {
        Square { num, picked: false }
    }
}

#[derive(Debug)]
struct BingoBoard {
    rows: Vec<Vec<Square>>,
    won: bool

}

impl BingoBoard {
    fn new() -> BingoBoard {
        BingoBoard {
            rows: Vec::new(),
            won: false
        } 
    }

    fn pick_square(&mut self, num: u8) -> bool {
        if self.won { return false; }

        for row in &mut self.rows {
            for square in row.iter_mut() {
                if num == square.num {
                    square.picked = true;
                    if self.check_win() {
                        self.won = true;
                        self.print_win(num);
                        return true;
                    }
                    return false;
                }
            }
        }
        false
    }

    fn check_win(&self) -> bool {
        if self.rows.iter().any(|row| {
            row.iter().all(|square| square.picked)
        }) {return true;}
                
        for i in 0..self.rows[0].len() {
            if self.rows.iter().all(|row| row[i].picked) {
                return true;
            }
        }
        false
    }

    fn print_win(&self, just_picked: u8) {
        println!("BOARD WON!");
        let mut sum_unpicked: u32 = 0;
        self.rows.iter().for_each(|row| {
            row.iter().for_each(|square| {
                if !square.picked {
                    sum_unpicked += square.num as u32;
                }
            })
        });
        println!("{}", sum_unpicked * just_picked as u32);
    }
}

fn day4_1() {
    let mut lines = read_lines("inputs/day4_input");
    let first_line = lines.next().unwrap().unwrap();
    let picked_nums = first_line.split(',')
                                .map(|val| val.parse().unwrap())
                                .collect::<Vec<u8>>();

    let mut boards: Vec<BingoBoard> = vec![];
    let mut curr_board: BingoBoard = BingoBoard::new();

    lines.next();
    for line in lines {
        let line = line.unwrap();
        if line.len() == 0 {
            boards.push(curr_board);
            curr_board = BingoBoard::new();
            continue
        };

        let row = line.split_whitespace()
                             .map(|s| Square::from(s.parse().unwrap()))
                             .collect::<Vec<Square>>();
        curr_board.rows.push(row);
    }
    boards.push(curr_board);

    for picked_num in picked_nums {
        if boards.iter_mut().any(|board| {
            board.pick_square(picked_num)
        }) {break};
    }

    
}

fn day4_2() {
    // let mut lines = read_lines("inputs/day4_input");
    let mut lines = read_lines("inputs/day4_lisa_input");

    let first_line = lines.next().unwrap().unwrap();
    let picked_nums = first_line.split(',')
                                .map(|val| val.parse().unwrap())
                                .collect::<Vec<u8>>();

    let mut boards: Vec<BingoBoard> = vec![];
    let mut curr_board: BingoBoard = BingoBoard::new();

    lines.next();
    for line in lines {
        let line = line.unwrap();
        if line.len() == 0 {
            boards.push(curr_board);
            curr_board = BingoBoard::new();
            continue
        };

        let row = line.split_whitespace()
                             .map(|s| Square::from(s.parse().unwrap()))
                             .collect::<Vec<Square>>();
        curr_board.rows.push(row);
    }
    boards.push(curr_board);

    for picked_num in picked_nums {
        boards.iter_mut().for_each(|board| {
            board.pick_square(picked_num);
        });
    }
}

fn main() {
    day4_2();
}