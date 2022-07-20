use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

#[derive(Debug, Clone)]
struct Matrix {
    data: Vec<i32>,
    rows: usize,
    cols: usize,
}

fn load_from_file(file_path: &str) -> Matrix {
    let file = File::open(file_path).expect(std::format!("File not found: {:}", file_path).as_str());
    let reader = BufReader::new(file);
    
    
    let mut res = Vec::new();
    let mut cols = 0usize;
    for line_res in reader.lines() {
        if let Ok(line) = line_res {
            cols = line.trim().len();
            for ch in line.trim().chars() {
                res.push(match ch {
                    '.' => 0,
                    '>' => 1,
                    'v' => 2,
                    _ => panic!("invalid char: '{:}'", ch),
                });
            }
        }
    }

    Matrix { rows: res.len() / cols, cols, data: res }
}

fn part1(mut matrix: Matrix) -> u64 {
    let mut buf = matrix.clone();
    
    let last_c = matrix.cols - 1;
    let last_r = matrix.rows - 1;
    
    let mut moved = true;
    let mut i = 0u64;
    while moved {
        moved = false;
        
        // move right
        for r in 0..matrix.rows {
            for c in 0..matrix.cols {
                let cur_idx = r * matrix.cols + c;
                if matrix.data[cur_idx] != 1 {
                    continue;
                }
                
                let next_c = if c == last_c { 0 } else { c + 1 };
                let next_idx = r * matrix.cols + next_c;
                if matrix.data[next_idx] == 0 {
                    moved = true;
                    buf.data[cur_idx] = 0;
                    buf.data[next_idx] = 1;
                }
            }
        }
        for i in 0..buf.data.len() {
            matrix.data[i] = buf.data[i];
        }
        
        // move down
        for r in 0..matrix.rows {
            for c in 0..matrix.cols {
                let cur_idx = r * matrix.cols + c;
                if matrix.data[cur_idx] != 2 {
                    continue;
                }
                
                let next_r = if r == last_r { 0 } else { r + 1 };
                let next_idx = next_r * matrix.cols + c;
                if matrix.data[next_idx] == 0 {
                    moved = true;
                    buf.data[cur_idx] = 0;
                    buf.data[next_idx] = 2;
                }
            }
        }
        for i in 0..buf.data.len() {
            matrix.data[i] = buf.data[i];
        }
        
        i += 1;
    }

    i
}

fn main() {
    let matrix = load_from_file("data.in");
    println!("{:?}", part1(matrix));
} 

