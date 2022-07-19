use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use regex::Regex;

#[derive(Debug, Clone)]
struct Line {
   x1: i32,
   y1: i32,
   x2: i32,
   y2: i32,
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}
        

fn load_from_file(file_path: &str) -> Vec<Line> {
    let file = File::open(file_path).expect("file wasn't found.");
    let reader = BufReader::new(file);
    
    let re = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$").unwrap();
    let mut result = Vec::new();
    for line in reader.lines() {
        let line_str = line.unwrap();
        let caps = re.captures(&line_str).unwrap();
        let mut line = Line { 
            x1: caps[1].parse::<i32>().unwrap(),
            y1: caps[2].parse::<i32>().unwrap(),
            x2: caps[3].parse::<i32>().unwrap(),
            y2: caps[4].parse::<i32>().unwrap(),
        };
        if line.x1 > line.x2 {
            std::mem::swap(&mut line.x1, &mut line.x2);
        }
        if line.y1 > line.y2 {
            std::mem::swap(&mut line.y1, &mut line.y2);
        }
        result.push(line);
    }

    result
}

fn main() {
    let raw_lines = load_from_file("data.in");
    let lines: Vec<Line> = raw_lines.iter().filter(|line| line.x1 == line.x2 || line.y1 == line.y2).cloned().collect();

    let mut res: HashMap<Point, i32> = HashMap::new();
    for line in &lines {
        if line.x1 == line.x2 {
            for i in line.y1..=line.y2 {
                let p = Point { x: line.x1, y: i };
                if let Some(&val) = res.get(&p) {
                    res.insert(p, val + 1);
                } else {
                    res.insert(p, 1);
                }
            }
        } else if line.y1 == line.y2 {
            for i in line.x1..=line.x2 {
                let p = Point { x: i, y: line.y1 };
                if let Some(&val) = res.get(&p) {
                    res.insert(p, val + 1);
                } else {
                    res.insert(p, 1);
                }
            }
        }
    }
    
    println!("{:}", res.iter().filter(|&(_, v)| *v >= 2).count());
}
