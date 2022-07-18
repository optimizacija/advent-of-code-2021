use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

enum Movement {
    Up,
    Down,
    Forward,
}

struct MoveAmount {
    movement: Movement,
    amount: i64,
}

fn load_from_file(file_path: &str) -> Vec<MoveAmount> {
    let file = File::open(file_path).expect("file wasn't found.");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.unwrap().split_whitespace().map(String::from).collect::<Vec<String>>())
        .map(|splits| MoveAmount { 
            movement: match splits[0].as_str() { 
                "forward" => Movement::Forward,
                "up" => Movement::Up,
                "down" => Movement::Down,
                _ => panic!("Invalid movement {:?}", splits),
            },
            amount: splits[1].parse::<i64>().unwrap() 
        })
        .collect()
}

fn main() {
    let move_amounts = load_from_file("data.in");
    let mut depth: i64 = 0;
    let mut forward: i64 = 0;
    
    for MoveAmount { movement, amount } in move_amounts {
        match movement {
            Movement::Down => depth += amount,
            Movement::Up => depth -= amount,
            Movement::Forward => forward += amount,
        }
    }
    
    println!("{:}", depth * forward);
}
