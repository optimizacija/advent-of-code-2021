use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use regex::Regex;

#[derive(Debug)]
struct Signal {
    input: Vec<String>,
    output: Vec<String>,
}

fn load_from_file(file_path: &str) -> Vec<Signal> {
    let file = File::open(file_path).expect(std::format!("File not found: {:}", file_path).as_str());
    let reader = BufReader::new(file);
    
    let re = Regex::new(r"^(.*) \| (.*)$").unwrap();
    let mut result = Vec::new();
    for line in reader.lines() {
        let line_str = line.unwrap();
        let caps = re.captures(&line_str).unwrap();
        result.push(Signal { 
            input: caps[1].split(' ').map(String::from).collect(),
            output: caps[2].split(' ').map(String::from).collect(),
        });
    } 
    
    result
}

fn count_base_codes(codes: &Vec<String>) -> i32 {
    let mut acc = 0;
    
    for code in codes {
        acc += match code.len() {
            2 | 3 | 4 | 7 => 1,
            _ => 0,
        };
    }
    
    acc
}


fn main() {
    let mut signals = load_from_file("data.in");
    let mut accumulator = 0;
    
    for signal in &mut signals {
        accumulator += count_base_codes(&signal.output);
    }
    
    println!("{:}", accumulator);
}
