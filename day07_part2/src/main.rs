use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn load_from_file(file_path: &str) -> Vec<i32> {
    let file = File::open(file_path).expect(std::format!("File not found: {:}", file_path).as_str());
    let mut reader = BufReader::new(file);
    
    let mut buf: String = String::new();
    reader.read_line(&mut buf).unwrap();
    
    buf.split(',').map(|val| val.trim().parse::<i32>().unwrap()).collect()
}

fn get_move_cost(crab: i32, pos: i32) -> i32 {
    let dist = (crab - pos).abs();
    dist * (dist + 1) / 2
}

fn get_align_cost(crabs: &Vec<i32>, pos: i32) -> i32 {
    crabs.iter().map(|&crab| get_move_cost(crab, pos)).sum()
}

fn main() {
    let crabs = load_from_file("data.in");
    
    let min = *crabs.iter().min().unwrap();
    let max = *crabs.iter().max().unwrap();
    let mut min_cost: i32 =  i32::MAX;
    for i in min..=max {
        let cost = get_align_cost(&crabs, i);
        if cost < min_cost {
            min_cost = cost;
        }
    }
    
    println!("{:}", min_cost);
}
