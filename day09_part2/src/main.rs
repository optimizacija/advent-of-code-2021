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

fn get_basin_product(map: &mut Map) -> i32 {
    let rows = map.data.len() / map.len;
    let cols = map.len;
    
    let mut res = Vec::new();
    for r in 0..rows {
        for c in 0..cols {
            let basin_size = flood_fill(map, r, c);
            if basin_size > 0 {
                res.push(basin_size);
            }
        }
    } 
    
    res.sort_unstable_by(|a, b| b.cmp(a));
    res[0] * res[1] * res[2]
}

fn flood_fill(map: &mut Map, r: usize, c: usize) -> i32 {
    if map.data[r * map.len + c] == 9 {
        0
    } else {
        map.data[r * map.len + c] = 9;
        
        let rows = map.data.len() / map.len;
        let cols = map.len;
        
        1 + 
        if r == 0 { 0 } else { flood_fill(map, r - 1, c) } + 
        if r >= (rows -1) { 0 } else { flood_fill(map, r + 1, c) } + 
        if c == 0 { 0 } else { flood_fill(map, r, c - 1) } + 
        if c >= (cols -1) { 0 } else { flood_fill(map, r, c + 1) }
    }
}

fn main() {
    let mut map = load_from_file("data.in");
    let basin_product = get_basin_product(&mut map);
    println!("{:}", basin_product);
}
