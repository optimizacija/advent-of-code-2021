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

fn has_line_error(line: &String) -> bool {
    let mut stack = Vec::new();
    
    for char in line.chars() {
        match char {
            '(' | '[' | '{' | '<' => stack.push(char),
            ')' | ']' | '}' | '>' => {
                match stack.pop() {
                    Some('(') => if char != ')' { return true },
                    Some('[') => if char != ']' { return true },
                    Some('{') => if char != '}' { return true },
                    Some('<') => if char != '>' { return true },
                    _ => return true,
                }
            }
            _ => return true,
        }
    }
    false
}

fn get_correction_score(lines: &Vec<String>) -> u64 {
    let valid_lines = lines.iter().filter(|line| !has_line_error(line)).collect::<Vec<&String>>();

    let mut score_vec = Vec::new();
    for line in &valid_lines {
        let mut stack = Vec::new();
        for char in line.chars() {
            match char {
                '(' | '[' | '{' | '<' => stack.push(char),
                ')' | ']' | '}' | '>' => { stack.pop(); () },
                _ => panic!("invalid char"),
            }
        }
        
        let mut line_score: u64 = 0;
        for char in stack.iter().rev() {
            match char {
                '(' => { line_score = line_score * 5 + 1; }
                '[' => { line_score = line_score * 5 + 2; }
                '{' => { line_score = line_score * 5 + 3; }
                '<' => { line_score = line_score * 5 + 4; }
                _ => panic!("invalid char"),
            }
        }
        score_vec.push(line_score);
    }
    
    score_vec.sort();
    score_vec[score_vec.len() / 2]
}

fn main() {
    let lines = load_from_file("data.in");
    let score = get_correction_score(&lines);
    println!("{:}", score);
}
