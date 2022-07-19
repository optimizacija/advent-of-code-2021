use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn load_from_file(file_path: &str) -> Vec<String> {
    let file = File::open(file_path).expect(std::format!("File not found: {:}", file_path).as_str());
    let reader = BufReader::new(file);
    
    let mut data = Vec::new();
    for line in reader.lines() {
        let line_str = line.unwrap();
        data.push(line_str);
    } 
    
    data
}

fn get_char_score(c: char) -> u64 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("invalid char"),
    }
}

fn get_error_score(lines: Vec<String>) -> u64 {
    let mut res: u64 = 0;

    for line in &lines {
        let mut stack = Vec::new();

        for char in line.chars() {
            match char {
                '(' | '[' | '{' | '<' => stack.push(char),
                ')' | ']' | '}' | '>' => {
                    match stack.pop() {
                        Some('(') => if char != ')' { res += get_char_score(char); break; },
                        Some('[') => if char != ']' { res += get_char_score(char); break; },
                        Some('{') => if char != '}' { res += get_char_score(char); break; },
                        Some('<') => if char != '>' { res += get_char_score(char); break; },
                        Some(x) => panic!("pop failed: {:}", x),
                        None => panic!("pop failed"),
                    }
                }
                _ => panic!("invalid char"),
            }
        }
    }

    res
}

fn main() {
    let lines = load_from_file("data.in");
    let score = get_error_score(lines);
    println!("{:}", score);
}
