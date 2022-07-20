use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use regex::Regex;

#[derive(Debug)]
struct Area {
    x1: i64,
    x2: i64,
    y1: i64,
    y2: i64,
}

impl Area {
    fn contains(self: &Self, point: &Point) -> bool {
        point.x >= self.x1 && point.x <= self.x2 && point.y >= self.y1 && point.y <= self.y2 
    }
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

fn load_from_file(file_path: &str) -> Area {
    let file = File::open(file_path).expect(std::format!("File not found: {:}", file_path).as_str());
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    
    let re = Regex::new(r"target area: x=([-]?\d+)\.\.([-]?\d+), y=([-]?\d+)\.\.([-]?\d+)").unwrap();
    let caps = re.captures(&line).unwrap();

    let x1 = caps.get(1).unwrap().as_str().parse::<i64>().unwrap();
    let x2 = caps.get(2).unwrap().as_str().parse::<i64>().unwrap();
    let y1 = caps.get(3).unwrap().as_str().parse::<i64>().unwrap();
    let y2 = caps.get(4).unwrap().as_str().parse::<i64>().unwrap();
    
    Area {
        x1: std::cmp::min(x1, x2),
        x2: std::cmp::max(x1, x2),
        y1: std::cmp::min(y1, y2),
        y2: std::cmp::max(y1, y2),
    }
}

fn try_hit(start_velocity: &Point, area: &Area) -> Option<i64> {
    let mut pos = Point { x: 0, y: 0 };
    let mut vel = start_velocity.clone();
    let mut max_y = 0;
    
    let min_y = std::cmp::min(area.y1, area.y2);
    while pos.y >= min_y {
        max_y = std::cmp::max(pos.y, max_y);
        
        if area.contains(&pos) {
            return Some(max_y);
        }
        
        pos.x += vel.x;
        pos.y += vel.y;
        vel.x = std::cmp::max(vel.x - vel.x.signum(), 0);
        vel.y -= 1;
    }
    
    None
}

fn count_viable_starting_velocities(area: &Area) -> i64 {
    let mut count = 0;
    for x in -2*area.x2..2*area.x2 {
        for y in (-2*area.y2.abs())..(2 * area.y2.abs()) {
            if let Some(_) = try_hit(&Point{ x, y }, &area) {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    let area = load_from_file("data.in");
    println!("{:}", count_viable_starting_velocities(&area));
}

