use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

type Score = [u64; 2];
type WinCount = [u64; 2];
type Position = [u64; 2];

fn load_from_file(file_path: &str) -> Position {
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


const MAX_SCORE: u64 = 21;

#[derive(Debug, Hash, PartialEq, Eq)]
struct Key {
    pos: Position,
    scores: Score,
    idx: usize,
}

fn get_dirac_dice_scores(cache: &mut HashMap<Key, Score>, pos: &mut Position, scores: &mut Score, curr_idx: usize) -> WinCount {
    let key = Key { pos: *pos, scores: *scores, idx: curr_idx };
    if let Some(score) = cache.get(&key) {
        return *score;
    }
    
    if let Some(idx) = scores.iter().position(|&score| score >= MAX_SCORE) {
        let mut result = [0u64; 2];
        result[idx] = 1;
        
        cache.insert(key, result);
        return result;
    }
    
    let mut result = [0, 0];
    let next_idx = (curr_idx + 1) % 2;
    for i in 1..=3 {
        for j in 1..=3 {
            for k in 1..=3 {
                let curr_score = i + j + k;
                let old_pos = pos[curr_idx];
                let old_score = scores[curr_idx];
                
                pos[curr_idx] = (pos[curr_idx] + curr_score - 1) % 10 + 1;
                scores[curr_idx] += pos[curr_idx];

                let win_count = get_dirac_dice_scores(cache, pos, scores, next_idx);
                
                pos[curr_idx] = old_pos;
                scores[curr_idx] = old_score;
                
                result[0] += win_count[0];
                result[1] += win_count[1];
            }
        }
    }
    
    cache.insert(key, result);
    result
}

fn main() {
    let mut input = load_from_file("data.in");
    let scores = get_dirac_dice_scores(&mut HashMap::new(), &mut input, &mut [0, 0], 0);
    println!("{:}", scores.iter().max().unwrap());
} 

