use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

#[derive(Debug)]
struct Map {
    data: Vec<i32>,
    len: usize,
}

fn load_from_file(file_path: &str) -> Map {
    let file = File::open(file_path).expect(std::format!("File not found: {:}", file_path).as_str());
    let reader = BufReader::new(file);
    
    let mut data = Vec::new();
    let mut len = 0;
    for line in reader.lines() {
        let line_str = line.unwrap();
        let mut line_nums = line_str.chars().map(|char| char.to_digit(10).unwrap() as i32).collect::<Vec<i32>>();
        len = line_nums.len();
        data.append(&mut line_nums);
    } 
    
    Map { data, len }
}

fn sum_risk_levels(map: &Map) -> i32 {
    let mut risk = 0;
    
    let rows = map.data.len() / map.len;
    for r in 0..rows {
        for c in 0..map.len {
            let cur = map.data[r * map.len + c];
            if r > 0 && map.data[(r - 1) * map.len + c] <= cur {
                continue;
            }
            if c > 0 && map.data[r * map.len + (c - 1)] <= cur {
                continue;
            }
            if r < (rows - 1) && map.data[(r + 1) * map.len + c] <= cur {
                continue;
            }
            if c < (map.len - 1) && map.data[r * map.len + (c + 1)] <= cur {
                continue;
            }
            risk += cur + 1;
        }
    }
    
    risk
}

fn main() {
    let map = load_from_file("data.in");
    let risk_levels = sum_risk_levels(&map);
    println!("{:}", risk_levels);
}
