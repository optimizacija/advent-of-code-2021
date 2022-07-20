use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

type Lookup = Vec<bool>;

#[derive(Debug, Clone)]
struct Matrix {
    data: Vec<bool>,
    rows: usize,
    cols: usize,
}

#[derive(Debug)]
struct Input {
    matrix: Matrix,
    lookup: Lookup,
}

fn convert_line_to_bool_vec(line: &String) -> Vec<bool> {
    line.trim().chars().map(|char| match char {
        '.' => false,
        '#' => true,
        _ => panic!("invalid char: '{:}'", char),
    }).collect::<Vec<bool>>()
}

fn load_from_file(file_path: &str) -> Input {
    let file = File::open(file_path).expect(std::format!("File not found: {:}", file_path).as_str());
    let mut reader = BufReader::new(file);
    let mut buf = String::new();

    reader.read_line(&mut buf).unwrap();
    let lookup = convert_line_to_bool_vec(&buf);
    
    reader.read_line(&mut buf).unwrap(); // empty line
    
    let mut data = Vec::new();
    let mut line_count = 0;
    for line_res in reader.lines() {
        if let Ok(line) = line_res {
            let mut values = convert_line_to_bool_vec(&line);
            data.append(&mut values);
            line_count += 1;
        }
    }

    Input {
        matrix: Matrix { cols: data.len() / line_count, data, rows: line_count},
        lookup,
    }
}

fn get_pixel_index(mat: &Matrix, r: i64, c: i64, oob_val: usize) -> usize {
    let mut val = 0;
    for rr in -1..=1 {
        for cc in -1..=1 {
            let rrr = r + rr;
            let ccc = c + cc;
            
            val <<= 1;
            
            if rrr >= 0 && 
                rrr < mat.rows as i64 &&
                ccc >= 0 && 
                ccc < mat.cols as i64 {
                // when in bounds, take the stored value
                val |= mat.data[rrr as usize * mat.rows + ccc as usize] as usize;
            } else {
                val |= oob_val;
            }
        }
    }
    val
}

fn step(i_mat: &Matrix, o_mat: &mut Matrix, lookup: &Lookup, oob_val: usize) {
    for r in 0..o_mat.rows as i64 {
        for c in 0..o_mat.cols as i64 {
            let rr = r - 1;
            let cc = c - 1;
            o_mat.data[r as usize * o_mat.rows + c as usize] = lookup[get_pixel_index(&i_mat, rr, cc, oob_val)];
        }
    }
}

fn count_lit_values(mat: &Matrix) -> usize {
    mat.data.iter().filter(|val| **val).count()
}

fn part_1(input: &Input) -> usize {
    let mut i_mat = input.matrix.clone();
    let mut o_mat = input.matrix.clone();
    o_mat.rows += 2;
    o_mat.cols += 2;
    o_mat.data.clear();
    o_mat.data.resize(o_mat.rows * o_mat.cols, false);

    let oob_vals = [input.lookup[0] as usize, input.lookup[511] as usize];
    let mut oob_idx = 0;
    
    for _ in 0..2 {
        step(&i_mat, &mut o_mat, &input.lookup, oob_idx);
        
        oob_idx = oob_vals[oob_idx];
        std::mem::swap(&mut i_mat, &mut o_mat);
        
        o_mat.rows += 4;
        o_mat.cols += 4;
        o_mat.data.clear();
        o_mat.data.resize(o_mat.rows * o_mat.cols, false);
    }

    count_lit_values(&i_mat)
}


fn main() {
    let input = load_from_file("data.in");
    println!("{:?}", part_1(&input));
} 

