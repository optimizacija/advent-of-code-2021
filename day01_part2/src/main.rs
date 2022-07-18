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
    for i in 3..numbers.len() {
        let prev_sum = numbers[i-3] + numbers[i-2] + numbers[i-1];
        let curr_sum = numbers[i-2] + numbers[i-1] + numbers[i];
        if curr_sum > prev_sum {
            res += 1;
        }
    }
    
    println!("{:}", res);
}
