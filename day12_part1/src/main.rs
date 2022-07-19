use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

type Connections = HashMap<String, Vec<String>>;

fn load_from_file(file_path: &str) -> Connections {
    let file = File::open(file_path).expect(std::format!("File not found: {:}", file_path).as_str());
    let reader = BufReader::new(file);
    
    let mut res: HashMap<String, Vec<String>> = HashMap::new();
    for line in reader.lines() {
        let line_str = line.unwrap();
        let split = line_str.split('-').collect::<Vec<&str>>();
        let left: String = split[0].to_string().clone();
        let right: String = split[1].to_string();
        
        if let Some(val) = res.get_mut(&left) {
            val.push(right.clone());
        } else {
            res.insert(left.clone(), vec![right.clone()]);
        }
        
        if let Some(val) = res.get_mut(&right) {
            val.push(left);
        } else {
            res.insert(right, vec![left]);
        }
    } 
    
    res
}

fn count_all_paths<'a, 'b>(connections: &'a Connections, buf: &'b mut Vec<&'a String>) -> i32 {
    let last = *buf.last().unwrap();
    if last.as_str() == "end" {
        return 1;
    }
    

    let caves = connections.get(last).unwrap();
    let mut count = 0;
    for cave in caves {
        if cave.chars().nth(0).unwrap().is_lowercase() {
            if buf.contains(&cave) {
                continue;
            }
        }
        buf.push(cave);
        count += count_all_paths(connections, buf);
        buf.pop();
    }
    
    return count;
}

fn main() {
    let connections = load_from_file("data.in");
    
    let mut buf = Vec::new();
    let start = String::from("start");
    buf.push(connections.get_key_value(&start).unwrap().0);
    let count = count_all_paths(&connections, &mut buf);
    
    println!("{:}", count);
}
