use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use regex::Regex;

/*
add a b - Add the value of a to the value of b, then store the result in variable a.
mul a b - Multiply the value of a by the value of b, then store the result in variable a.
div a b - Divide the value of a by the value of b, truncate the result to an integer, then store the result in variable a. (Here, "truncate" means to round the value toward zero.)
mod a b - Divide the value of a by the value of b, then store the remainder in variable a. (This is also called the modulo operation.)
eql a b - If the value of a and b are equal, then store the value 1 in variable a. Otherwise, store the value 0 in variable a.
*/

#[derive(Debug)]
enum LiteralOrIndex {
    Literal(i64),
    Index(usize),
}

#[derive(Debug)]
enum Instruction {
    Input(usize),
    Add(usize, LiteralOrIndex),
    Mul(usize, LiteralOrIndex),
    Div(usize, LiteralOrIndex),
    Mod(usize, LiteralOrIndex),
    Eql(usize, LiteralOrIndex),
}

fn load_from_file(file_path: &str) -> Vec<Instruction> {
    let file = File::open(file_path).expect(std::format!("File not found: {:}", file_path).as_str());
    let reader = BufReader::new(file);
    
    let re = Regex::new(r"(\w+) (\w) *(-?\w+)?").unwrap();
    
    let mut res = Vec::new();
    for line_res in reader.lines() {
        if let Ok(line) = line_res {
            let caps = re.captures(&line).unwrap();
            res.push(match &caps[1] {
                "inp" => Instruction::Input(get_var_offset(&caps[2])),
                "add" => Instruction::Add(get_var_offset(&caps[2]), get_literal_or_index(&caps[3])),
                "mul" => Instruction::Mul(get_var_offset(&caps[2]), get_literal_or_index(&caps[3])),
                "div" => Instruction::Div(get_var_offset(&caps[2]), get_literal_or_index(&caps[3])),
                "mod" => Instruction::Mod(get_var_offset(&caps[2]), get_literal_or_index(&caps[3])),
                "eql" => Instruction::Eql(get_var_offset(&caps[2]), get_literal_or_index(&caps[3])),
                _ => panic!("unknown capture"),
            });
        }
    }

    res
}

fn get_literal_or_index(cap: &str) -> LiteralOrIndex {
    match cap.parse::<i64>() {
        Ok(num) => LiteralOrIndex::Literal(num),
        Err(_) => LiteralOrIndex::Index(get_var_offset(cap)),
    }
}

fn get_var_offset(cap: &str) -> usize {
    match cap {
        "x" => 0,
        "y" => 1,
        "z" => 2,
        "w" => 3,
        _ => panic!("unknown capture"),
    }
}

fn interpret(instructions: &Vec<Instruction>, vars: &mut [i64; 4], value: i64) -> bool {
    let mut msb_digit_idx = 0u32;
    for instruction in instructions {
        match instruction {
            Instruction::Input(offset) => { 
                vars[*offset] = get_digit(value, msb_digit_idx);
                msb_digit_idx += 1;
            },
            Instruction::Add(offset, literal_or_index) => {
                vars[*offset] += get_value(vars, &literal_or_index);
            },
            Instruction::Mul(offset, literal_or_index) => {
                vars[*offset] *= get_value(vars, &literal_or_index);
            },
            Instruction::Div(offset, literal_or_index) => {
                vars[*offset] /= get_value(vars, &literal_or_index);
            },
            Instruction::Mod(offset, literal_or_index) => {
                vars[*offset] %= get_value(vars, &literal_or_index);
            },
            Instruction::Eql(offset, literal_or_index) => {
                vars[*offset] = if vars[*offset] == get_value(vars, &literal_or_index) { 1 } else { 0 };
            },
        }
    }
    
    vars[2] == 0 
}

fn get_value(vars: &[i64; 4], literal_or_index: &LiteralOrIndex) -> i64 {
    match literal_or_index {
        LiteralOrIndex::Literal(literal) => *literal,
        LiteralOrIndex::Index(offset) => vars[*offset],
    }
}

fn get_digit(mut value: i64, msb_digit_idx: u32) -> i64 {
    value /= 10i64.pow(13 - msb_digit_idx);
    value % 10
}

fn part1(instructions: &Vec<Instruction>) -> i64 {
    let mut val = 99999999999999;
    let mut z = i64::MAX;
    
    let mut vars = [0i64; 4];
    loop {
        let mut min_val = i64::MAX;
        let mut min_z = i64::MAX;
        for digit_i in 0usize..14 {
            for digit in ('1'..='9').rev() {
                let mut digits = val.to_string();
                digits.replace_range(digit_i..=digit_i, &digit.to_string());
                let new_val = digits.parse::<i64>().unwrap();
                
                interpret(instructions, &mut vars, new_val);
                if vars[2] < min_z {
                    min_z = vars[2];
                    min_val = new_val;
                }
                vars.fill(0);
            }
        }
        
        if min_z >= z || min_z == 0 {
            return min_val;
        }
        
        val = min_val;
        z = min_z;
    }
}

fn main() {
    let instructions = load_from_file("data.in");
    println!("{:?}", part1(&instructions));
} 

