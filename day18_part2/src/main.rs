use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

#[derive(Debug, Clone, Copy)]
struct Regular {
    value: u32,
    depth: u32,
}

type Regulars = Vec<Regular>;


fn load_from_file(file_path: &str) -> Vec<Regulars> {
    let file = File::open(file_path).expect(std::format!("File not found: {:}", file_path).as_str());
    let reader = BufReader::new(file);

    let mut result = Vec::new();
    for line in reader.lines() {
        let mut regulars = Regulars::new();

        let mut depth = 0;
        for c in line.unwrap().chars() {
            match c {
                '[' => depth += 1,
                ']' => depth -= 1,
                ',' => (),
                x => regulars.push(Regular {value: x.to_digit(10).unwrap(), depth}),
            }
        }
        result.push(regulars);
    }

    result
}

fn add_regulars(destination: &mut Regulars, source: &Regulars) {
    destination.append(& mut source.clone());
    for mut regular in destination {
        regular.depth += 1;
    }
}

fn must_explode(regulars: &Regulars) -> bool {
    regulars.iter().any(|regular| regular.depth > 4)
}

fn must_split(regulars: &Regulars) -> bool {
    regulars.iter().any(|regular| regular.value >= 10)
}

fn is_regularized(regulars: &Regulars) -> bool {
    !must_explode(regulars) && !must_split(regulars)
}

fn explode(regulars: &mut Regulars) {
    // explode everything that has to be exploded (left -> right)
    while must_explode(regulars) {
        let len = regulars.len() - 1;
        for i in 0..len {
            if regulars[i].depth > 4 {
                if i > 0 {
                    regulars[i - 1].value += regulars[i].value;
                } 
                if (i + 2) <= len {
                    regulars[i + 2].value += regulars[i + 1].value;
                }
                
                regulars[i].value = 0;
                regulars[i].depth -= 1;
                regulars.remove(i + 1);
                break;
            }
        }
    }
}

fn split(regulars: &mut Regulars) {
    // split only 1 element
    let i = regulars.iter().position(|regular| regular.value >= 10).unwrap();
    regulars[i].depth += 1;
    let temp = regulars[i].value;
    regulars[i].value = temp / 2;
    regulars.insert(i + 1, Regular { value: temp - regulars[i].value, depth: regulars[i].depth });
}

fn add_all_regulars(mut regulars_vec: Vec<Regulars>) -> Regulars {
    let mut result: Regulars = Vec::new();
    result.append(&mut regulars_vec[0].clone());
    regulars_vec.remove(0);

    for regulars in regulars_vec {
        add_regulars(&mut result, &regulars);
        
        while !is_regularized(&result) {
            if must_explode(&result) {
                explode(&mut result);
            } 
            if must_split(&result) {
                split(&mut result);
            }
        }
    }

    result
}

fn get_regular_sum(mut regulars: Regulars) -> u32 {
    while regulars.len() != 1 {
        let max_depth = regulars.iter().map(|regular| regular.depth).max().unwrap();
        
        let mut reg = Vec::new();
        let len = regulars.len();
        let mut i = 0;
        while i < len {
            if regulars[i].depth == max_depth {
                reg.push(Regular {
                    value: 3 * regulars[i].value + 2 * regulars[i + 1].value,
                    depth: regulars[i].depth - 1,
                });
                i += 1;
            } else {
                reg.push(regulars[i]);
            }
            i += 1;
        }
        
        regulars = reg;
    }
    
    regulars.first().unwrap().value
}

fn get_max_magnitude(regulars_vec: Vec<Regulars>) -> u32 {
    let mut max = 0;
    
    for i in 0..regulars_vec.len() {
        for j in 0..regulars_vec.len() {
            let mut vec = Vec::new();
            vec.push(regulars_vec[i].clone());
            vec.push(regulars_vec[j].clone());
            
            let regulars = add_all_regulars(vec);
            
            let sum = get_regular_sum(regulars);
            if sum > max {
                max = sum;
            }
        }
    }
    
    max
}

fn main() {
    let regulars_vec = load_from_file("data.in");
    println!("{:}", get_max_magnitude(regulars_vec));
} 

