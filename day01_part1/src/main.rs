use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn load_from_file(file_path: &str) -> Vec<i64> {
    let file = File::open(file_path).expect("file wasn't found.");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.unwrap().parse::<i64>().unwrap())
        .collect()
}

fn main() {
    let numbers = load_from_file("data.in");
    let mut res: i64 = 0;
    for i in 1..numbers.len() {
        if numbers[i] > numbers[i-1] {
            res += 1;
        }
    }
    
    println!("{:}", res);
}
