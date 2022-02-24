use advent::read_lines;

fn day2_1() {
    struct Position {
        horizontal: i32,
        depth: i32,
    }
    let mut pos = Position {horizontal: 0, depth: 0};
    let lines = read_lines("inputs/day2_input");
    for line in lines {
        let line = line.unwrap();
        let line = line.split(' ').collect::<Vec<&str>>();
        let val = line[1].parse::<i32>().unwrap();
        match line[0] {
            "forward" => pos.horizontal += val,
            "down" => pos.depth += val,
            "up" => pos.depth -= val,
            _ => ()
        }

    }
    println!("{}", pos.horizontal * pos.depth);
}

fn day2_2() {
    struct Position {
        horizontal: i32,
        depth: i32,
        aim: i32,
    }
    let mut pos = Position {horizontal: 0, depth: 0, aim: 0};
    let lines = read_lines("inputs/day2_input");
    for line in lines {
        let line = line.unwrap();
        let line = line.split(' ').collect::<Vec<&str>>();
        let val = line[1].parse::<i32>().unwrap();
        match line[0] {
            "forward" => {
                pos.horizontal += val;
                pos.depth += val * pos.aim;
            },
            "down" => pos.aim += val,
            "up" => pos.aim -= val,
            _ => ()
        }

    }
    println!("{}", pos.horizontal * pos.depth);

}

fn main() {

}