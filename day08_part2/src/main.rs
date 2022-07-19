use std::collections::HashMap;
use std::collections::HashSet;
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

fn get_codes(signal: &Signal) -> Vec<String> {
    let mut codes = Vec::new();
    codes.extend(signal.input.iter().cloned());
    codes.extend(signal.output.iter().cloned());
    codes
}

fn sort_codes(codes: &mut Vec<String>) {
    for code in codes {
        let mut l: Vec<char> = code.chars().collect();
        l.sort();
        *code = l.into_iter().collect();
    }
}

fn contains_code(src_code: &String, code: &String) -> bool {
    code.chars().all(|char| src_code.contains(char))
}

fn add_base_codes(num_to_code: &mut HashMap::<i32, String>, codes: &Vec<String>) {
    for code in codes {
        match code.len() {
            2 => { num_to_code.insert(1, code.clone()); () },
            3 => { num_to_code.insert(7, code.clone()); () },
            4 => { num_to_code.insert(4, code.clone()); () },
            7 => { num_to_code.insert(8, code.clone()); () },
            _ => (),
        }
    }
}

fn add_len_5_codes(num_to_code: &mut HashMap::<i32, String>, codes: &Vec<String>) {
    let len_6_codes = codes
        .iter()
        .filter(|code| code.len() == 6)
        .cloned()
        .collect::<HashSet<String>>()
        .into_iter()
        .collect::<Vec<String>>();
    
    codes
        .iter()
        .filter(|code| code.len() == 5)
        .for_each(|code| {
            if !len_6_codes.iter().any(|l6code| contains_code(l6code, code)) {
                num_to_code.insert(2, code.clone());
            } else if contains_code(code, &num_to_code[&7]) {
                num_to_code.insert(3, code.clone());
            } else {
                num_to_code.insert(5, code.clone());
            }
        });
}

fn add_len_6_codes(num_to_code: &mut HashMap::<i32, String>, codes: &Vec<String>) {
    codes
        .iter()
        .filter(|code| code.len() == 6)
        .for_each(|code| {
            if contains_code(code, &num_to_code[&3]) {
                num_to_code.insert(9, code.clone());
            } else if contains_code(code, &num_to_code[&5]) {
                num_to_code.insert(6, code.clone());
            } else {
                num_to_code.insert(0, code.clone());
            }
        });
}

fn get_output_str(num_to_code: &HashMap::<i32, String>, codes: &Vec<String>) -> String {
    let code_to_num: HashMap<&String, &i32> = Vec::from_iter(num_to_code.iter())
        .iter()
        .clone()
        .map(|(k, v)| (v.clone(), k.clone()))
        .collect();

    codes.iter().map(|code| code_to_num[code].to_string()).collect::<String>()
}

fn main() {
    let mut signals = load_from_file("data.in");
    let mut accumulator = 0;
    
    for signal in &mut signals {
        sort_codes(&mut signal.input);
        sort_codes(&mut signal.output);
        let codes = get_codes(&signal);
        
        let mut num_to_code = HashMap::<i32, String>::new();
        add_base_codes(&mut num_to_code, &codes);
        add_len_5_codes(&mut num_to_code, &codes);
        add_len_6_codes(&mut num_to_code, &codes);

        let output_str = get_output_str(&num_to_code, &signal.output);
        accumulator += output_str.parse::<i32>().unwrap();
    }
    
    println!("{:}", accumulator);
}

/*
 num: is in | is not in
   1: 0 1 3 4 7 8 9 | 2 5 6
   2: 2 8 | 0 1 3 4 5 6 7 9
   3: 3 8 9 | 0 2 3 4 5 6 7 
   4: 4 8 9 | 0 1 2 3 5 6 7 
   5: 5 6 8 | 0 1 2 3 4 7 9
   6: 6 8 | 0 1 2 3 4 5 7 9
   7: 0 3 7 8 9 | 1 2 4 5 6
   8: 8 | 0 1 2 3 4 5 6 7 9
   9: 8 9 | 0 1 2 3 4 5 6 7
   0: 0 8 | 1 2 3 4 5 6 7 9

 num: contains | does not contain
   1: 1 | 2 3 4 5 6 7 8 9 0
   2: 2 | 1 3 4 5 6 7 8 9 0
   3: 1 3 7 | 2 4 5 6 8 9 0
   4: 1 4 | 2 3 5 6 7 8 9 0
   5: 5 | 1 2 3 5 6 7 8 9 0
   6: 5 6 | 1 2 3 4 7 8 9 0
   7: 1 7 | 2 3 4 5 6 8 9 0
   8: 1 2 3 4 5 6 7 8 9 0 | 
   9: 1 3 4 5 7 9 | 2 6 8 0
   0: 1 7 | 2 3 4 5 6 8 0 9 

2: 1
3: 7
4: 4
5: 2 3 5 
6: 6 9 0
7: 8

1st pass
1 cf
7 acf
4 bcdf
8 abcdefg

2nd pass
5: 2 3 5 
2 acdeg <- is not in any 6-len num
3 acdfg <- contains 7
5 abdfg <- otherwise 

3rd pass
6: 6 9 0
6 abdefg <- contains 5 (& does not contain 3)
9 abcdfg <- contains 3
0 abcefg <- contains 7 (& does not contain 3)
 */
