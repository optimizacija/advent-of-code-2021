use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use regex::Regex;

#[derive(Debug)]
struct PolymerConfig {
    template: String,
    inserts: HashMap<(char, char), char>,
}

fn load_from_file(file_path: &str) -> PolymerConfig {
    let file = File::open(file_path).expect(std::format!("File not found: {:}", file_path).as_str());
    let reader = BufReader::new(file);
    
    let mut lines = reader.lines();
    let template: String = lines.next().unwrap().unwrap();
    lines.next().unwrap().unwrap();
    
    let re = Regex::new(r"^.*(\w)(\w) -> (\w)$").unwrap();
    let inserts = lines
        .into_iter()
        .map(|line| {
            let line_str = line.unwrap();
            let caps = re.captures(&line_str).unwrap();
            
            (
                (
                    caps[1].chars().nth(0).unwrap(),
                    caps[2].chars().nth(0).unwrap(),
                ),
                caps[3].chars().nth(0).unwrap(),
            )
        })
        .collect::<HashMap<(char, char), char>>();
        
    PolymerConfig { template, inserts }
}

fn get_specified_quantaty(polymer_config: &PolymerConfig) -> i64 {
    let mut buf: HashMap<(char, char), i64> = HashMap::new();
    let mut buf2: HashMap<(char, char), i64> = HashMap::new();
    
    for i in 1..polymer_config.template.len() {
        let prev = polymer_config.template.chars().nth(i-1).unwrap();
        let curr = polymer_config.template.chars().nth(i).unwrap();
        *buf.entry((prev, curr)).or_insert(0) += 1;
    }
    // starting & terminal chars
    buf.insert(('-', polymer_config.template.chars().next().unwrap()), 1);
    buf.insert((polymer_config.template.chars().last().unwrap(), '-'), 1);
    
    let steps = 40;
    for _ in 1..=steps {
        for (key, value) in &buf {
            if let Some(val) = polymer_config.inserts.get(&key) {
                *buf2.entry((key.0, *val)).or_insert(0) += value;
                *buf2.entry((*val, key.1)).or_insert(0) += value;
            } else {
                *buf2.entry(*key).or_insert(0) += value;
            }
        }
        buf = buf2.clone();
        buf2.clear();
    }
    
    let mut count_map: HashMap<char, i64> = HashMap::new();

    for (key, value) in &buf {
        *count_map.entry(key.0).or_insert(0) += value;
        *count_map.entry(key.1).or_insert(0) += value;
    }
    
    count_map.remove(&'-');

    let least_common_char = count_map.values().min().unwrap() / 2;
    let most_common_char = count_map.values().max().unwrap() / 2;
        
    most_common_char - least_common_char
} 

fn main() {
    let polymer_config = load_from_file("data.in");
    let size = get_specified_quantaty(&polymer_config);
    println!("{:?}", size);
}
