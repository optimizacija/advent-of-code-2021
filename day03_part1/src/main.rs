use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn load_from_file(file_path: &str) -> Vec<String> {
    let file = File::open(file_path).expect("file wasn't found.");
    let reader = BufReader::new(file);

    reader.lines().map(|line| line.unwrap()).collect()
}

fn get_msb_value(lines: &Vec<String>) -> usize {
    let line_len = lines[0].len();
    let mut bit_1_counts: Vec<usize> = vec![0; line_len];

    for line in lines {
        let line_bytes = line.as_bytes();
        for i in 0..line_len {
            if line_bytes[i] == "1".as_bytes()[0] {
                bit_1_counts[i] += 1;
            }
        }
    }
    
    let half_lines: usize = lines.len() / 2;
    let mut result: usize = 0;
    for bit_1_count in bit_1_counts {
        result <<= 1;
        result |= (bit_1_count > half_lines) as usize;
    }

    result
}

fn negate_bits_with_len(val: usize, len: u32) -> usize {
    let shift_len = usize::BITS - len;
   (!val << shift_len) >> shift_len
}

fn main() {
    let lines = load_from_file("data.in");
    let msb_value: usize = get_msb_value(&lines);
    let lsb_value: usize = negate_bits_with_len(msb_value, lines[0].len() as u32);
    println!("{:}", lsb_value * msb_value);
}
