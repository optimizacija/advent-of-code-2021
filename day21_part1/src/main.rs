use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn load_from_file(file_path: &str) -> [u64; 2] {
    let file = File::open(file_path).expect(std::format!("File not found: {:}", file_path).as_str());
    let reader = BufReader::new(file);

    let mut res = [0u64; 2];
    let mut i = 0;
    for line_res in reader.lines() {
        if let Ok(line) = line_res {
            res[i] = line.chars().last().unwrap().to_digit(10).unwrap() as u64;
        }
        i += 1;
    }

    res
}

fn get_dirac_dice_score(mut pos: [u64; 2]) -> u64 {
    let mut scores = [0u64; 2];
    let mut p_i = 0;
    
    let mut dice_val = 1;
    let mut die_roll = 0;
    
    const MAX_SCORE: u64 = 1000;
    while !scores.iter().any(|&score| score >= MAX_SCORE) {
        let mut curr_score = 0;
        for _ in 0..3 {
            curr_score += dice_val;
            dice_val += 1;
        }
        die_roll += 3;
        
        curr_score = (curr_score + pos[p_i] - 1) % 10 + 1;
        pos[p_i] = curr_score;
        scores[p_i] += curr_score;
        
        p_i = (p_i + 1) % 2;
    }
    
    scores.iter().min().unwrap() * die_roll
}

fn main() {
    let input = load_from_file("data.in");
    println!("{:}", get_dirac_dice_score(input));
} 

