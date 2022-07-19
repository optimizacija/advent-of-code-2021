use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

#[derive(Debug)]
struct Map {
    data: Vec<i32>,
    cols: usize,
    rows: usize,
}

fn load_from_file(file_path: &str) -> Map {
    let file = File::open(file_path).expect(std::format!("File not found: {:}", file_path).as_str());
    let reader = BufReader::new(file);
    
    let mut data = Vec::new();
    let mut cols = 0;
    for line in reader.lines() {
        let line_str = line.unwrap();
        let mut line_nums = line_str.chars().map(|char| char.to_digit(10).unwrap() as i32).collect::<Vec<i32>>();
        cols = line_nums.len();
        data.append(&mut line_nums);
    } 
    
    Map { rows: data.len() / cols, data, cols}
}

fn simulate_flashes(map: &mut Map) -> usize {
    for i in 1.. {
        increase_energy_levels(map);
        while should_flash(map) {
            flash(map);
        }
        reset_flash(map);
        if count_flashes(map) == map.data.len() {
            return i;
        }
    }
    panic!("unreachable code");
}

fn increase_energy_levels(map: &mut Map) {
    for element in &mut map.data {
        *element += 1;
    }
}

fn should_flash(map: &Map) -> bool {
    map.data.iter().any(|&val| val > 9)
}

fn flash(map: &mut Map) {
    let cols = map.cols as i32;
    let rows = map.rows as i32;

    let mut data_copy = map.data.clone();
    for r in 0..rows {
        for c in 0..cols {
            let mut flash_count = 0;
            
            let idx = (r * cols + c) as usize;
            if map.data[idx] == -1 {
                continue;
            }
            
            for y in -1..=1 {
                for x in -1..=1 {
                    let rr = r + y;
                    let cc = c + x;
                    
                    if cc >= 0 && rr >= 0 && cc < cols && rr < rows {
                        flash_count += (map.data[(rr * cols + cc) as usize] > 9) as i32;
                    }
                }
            }
            
            data_copy[idx] = map.data[idx] + flash_count;
        }
    }
    
    for i in 0..map.data.len() {
        if map.data[i] > 9 {
            map.data[i] = -1;
        } else {
            map.data[i] = data_copy[i];
        }
    }
}

fn reset_flash(map: &mut Map) {
    for ele in &mut map.data {
        if *ele == -1 {
            *ele = 0;
        }
    }
}

fn count_flashes(map: &mut Map) -> usize {
    map.data.iter().filter(|&&val| val == 0).count()
}

fn main() {
    let mut map = load_from_file("data.in");
    let score = simulate_flashes(&mut map);
    println!("{:}", score);
}
