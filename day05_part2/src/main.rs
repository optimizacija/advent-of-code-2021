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
        result.push(Line { 
            x1: caps[1].parse::<i32>().unwrap(),
            y1: caps[2].parse::<i32>().unwrap(),
            x2: caps[3].parse::<i32>().unwrap(),
            y2: caps[4].parse::<i32>().unwrap(),
        });
    }

    result
}

fn main() {
    let raw_lines = load_from_file("data.in");
    let lines: Vec<Line> = raw_lines
        .iter()
        .filter(|line| line.x1 == line.x2 || 
                line.y1 == line.y2 ||
                (line.x2 - line.x1).abs() == (line.y2 - line.y1).abs())
        .cloned()
        .collect();

    let mut res: HashMap<Point, i32> = HashMap::new();
    for line in &lines {
        let dxx: i32 = line.x2 - line.x1;
        let dyy: i32 = line.y2 - line.y1;
        let diff: i32 = (if dxx != 0 { dxx } else { dyy }).abs();
        let dx: i32 = (line.x2 - line.x1).clamp(-1, 1);
        let dy: i32 = (line.y2 - line.y1).clamp(-1, 1);

        for d in 0..=diff {
            let p = Point { 
                x: line.x1 + dx * d,
                y: line.y1 + dy * d,
            };
            if let Some(&val) = res.get(&p) {
                res.insert(p, val + 1);
            } else {
                res.insert(p, 1);
            }
        }
    }
    
    println!("{:}", res.iter().filter(|&(_, v)| *v >= 2).count());
}
