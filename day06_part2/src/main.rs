use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn load_from_file(file_path: &str) -> Vec<u32> {
    let file = File::open(file_path).expect(std::format!("File not found: {:}", file_path).as_str());
    let mut reader = BufReader::new(file);
    
    let mut buf: String = String::new();
    reader.read_line(&mut buf).unwrap();
    
    buf.split(',').map(|val| val.trim().parse::<u32>().unwrap()).collect()
}

fn main() {
    let fish = load_from_file("data.in");
    
    let mut buf: Vec<u64> = vec![0;9];
    for f in &fish {
        buf[*f as usize] += 1;
    }

    for _ in 0..256 {
        let new_count = buf[0];
        for i in 1..buf.len() {
            buf[i-1] = buf[i];
        }
        buf[6] += new_count;
        buf[8] = new_count;
        
    }
    
    println!("{:}", buf.iter().sum::<u64>());
}
