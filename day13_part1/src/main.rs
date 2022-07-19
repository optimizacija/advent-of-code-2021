use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use regex::Regex;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum FoldDir {
    X,
    Y,
}

#[derive(Debug)]
struct Fold {
    val: i32,
    dir: FoldDir,
}

#[derive(Debug)]
struct Instructions {
    points: Vec<Point>,
    folds: Vec<Fold>,
}

fn load_from_file(file_path: &str) -> Instructions {
    let file = File::open(file_path).expect(std::format!("File not found: {:}", file_path).as_str());
    let reader = BufReader::new(file);
    
    let mut points: Vec<Point> = Vec::new();
    let mut lines = reader.lines();
    for line in &mut lines {
        let line_str = line.unwrap();
        if line_str.trim() == "" {
            break;
        }
        
        let split = line_str.split(',').map(|split_str| split_str.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        points.push(Point { x: split[0], y: split[1] });
    } 
    
    
    let re = Regex::new(r"^.*([xy])=(\d+)$").unwrap();
    let mut folds: Vec<Fold> = Vec::new();
    for line in &mut lines {
        let line_str = line.unwrap();
        let caps = re.captures(&line_str).unwrap();
        folds.push(
            match &caps[1] {
                "x" => Fold{ val: caps[2].parse::<i32>().unwrap(), dir: FoldDir::X },
                "y" => Fold{ val: caps[2].parse::<i32>().unwrap(), dir: FoldDir::Y },
                _ => panic!("unknown capture {:?}", &caps[1]),
            }
        )
    } 
    
    Instructions { points, folds }
}


fn fold_dots(instructions: &mut Instructions) {
    for fold in &instructions.folds {
        for point in &mut instructions.points {
            match fold.dir {
                FoldDir::X => if point.x >= fold.val { point.x = fold.val - (point.x - fold.val); },
                FoldDir::Y => if point.y >= fold.val { point.y = fold.val - (point.y - fold.val); },
            }
        }
    }
}

fn count_dots(instructions: &Instructions) -> i32 {
    instructions.points.iter().collect::<HashSet<&Point>>().iter().count() as i32
}

fn fold_and_count_dots(instructions: &mut Instructions) -> i32 {
    fold_dots(instructions);
    count_dots(instructions)
}

fn main() {
    let mut instructions = load_from_file("data.in");
    let count  = fold_and_count_dots(&mut instructions);
    println!("{:}", count);
}
