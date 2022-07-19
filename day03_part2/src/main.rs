use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn load_from_file(file_path: &str) -> Vec<String> {
    let file = File::open(file_path).expect("file wasn't found.");
    let reader = BufReader::new(file);

    reader.lines().map(|line| line.unwrap()).collect()
}

fn get_ogr(lines: &Vec<String>) -> String {
    let line_len = lines[0].len();
    let mut filtered_lines: Vec<&String> = lines.iter().collect();
    for i in 0..line_len {
        let f_line_len = filtered_lines.len();
        if f_line_len == 1 {
            return filtered_lines[0].to_string();
        }
        
        let mut set_bit_count = 0;
        for f_line in &filtered_lines {
            let line_bytes = f_line.as_bytes();
            if line_bytes[i] == "1".as_bytes()[0] {
                set_bit_count += 1;
            }
        }
        
        if set_bit_count >= (f_line_len - set_bit_count) {
            filtered_lines = filtered_lines.into_iter().filter(|line| line.as_bytes()[i] == "1".as_bytes()[0]).collect();
        } else {
            filtered_lines = filtered_lines.into_iter().filter(|line| line.as_bytes()[i] == "0".as_bytes()[0]).collect();
        }
    }

    filtered_lines[0].to_string()
}

fn get_csr(lines: &Vec<String>) -> String {
    let line_len = lines[0].len();
    let mut filtered_lines: Vec<&String> = lines.iter().collect();
    for i in 0..line_len {
        let f_line_len = filtered_lines.len();
        if f_line_len == 1 {
            return filtered_lines[0].to_string();
        }
        
        let mut unset_bit_count = 0;
        for f_line in &filtered_lines {
            let line_bytes = f_line.as_bytes();
            if line_bytes[i] == "0".as_bytes()[0] {
                unset_bit_count += 1;
            }
        }
        
        if unset_bit_count <= (f_line_len - unset_bit_count) {
            filtered_lines = filtered_lines.into_iter().filter(|line| line.as_bytes()[i] == "0".as_bytes()[0]).collect();
        } else {
            filtered_lines = filtered_lines.into_iter().filter(|line| line.as_bytes()[i] == "1".as_bytes()[0]).collect();
        }
    }

    filtered_lines[0].to_string()
}

fn main() {
    let lines = load_from_file("data.in");
    let ogr: String = get_ogr(&lines);
    let csr: String = get_csr(&lines);
    let ogr_val = isize::from_str_radix(&ogr, 2).unwrap();
    let csr_val = isize::from_str_radix(&csr, 2).unwrap();
    println!("{:}", ogr_val * csr_val);
}
