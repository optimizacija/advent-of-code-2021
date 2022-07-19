use std::collections::HashMap;
use std::collections::LinkedList;
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
    let mut buf: LinkedList<char> = polymer_config.template.chars().collect();
    let mut buf2: LinkedList<char> = LinkedList::new();
    let steps = 10;
    for _ in 1..=steps {
        let mut prev = buf.pop_front().unwrap();
        for curr in &buf {
            buf2.push_back(prev);
            if let Some(val) = polymer_config.inserts.get(&(prev, *curr)) {
                buf2.push_back(*val);
            }
            prev = *curr;
        }
        buf2.push_back(prev);
        
        buf.clear();
        buf.append(&mut buf2);
    }
    
    let count_map: HashMap<&char, i64> = buf.iter().fold(HashMap::new(), |mut acc, current| {
        *acc.entry(current).or_insert(0) += 1;
        acc
    });
    
    let least_common_char = count_map.values().min().unwrap();
    let most_common_char = count_map.values().max().unwrap();
        
    most_common_char - least_common_char
} 

fn main() {
    let polymer_config = load_from_file("data.in");
    let size = get_specified_quantaty(&polymer_config);
    println!("{:?}", size);
}
