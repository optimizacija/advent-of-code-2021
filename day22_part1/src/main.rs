use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use regex::Regex;

#[derive(Debug, Clone, Copy)]
struct Cuboid {
    x1: i64,
    x2: i64,
    y1: i64,
    y2: i64,
    z1: i64,
    z2: i64,
    sign: bool,
}

impl Cuboid {
    fn volume(self: &Self) -> i64 {
        // sign: true -> 1, false -> -1
        (self.sign as i64 * 2 - 1) * 
        // volume: a * b * c
        (self.x2 - self.x1 + 1) * (self.y2 - self.y1 + 1) * (self.z2 - self.z1 + 1) 
    }
}

#[derive(Hash, PartialEq, Eq)]
struct Cube(i64, i64, i64);

fn load_from_file(file_path: &str) -> Vec<Cuboid> {
    let file = File::open(file_path).expect(std::format!("File not found: {:}", file_path).as_str());
    let reader = BufReader::new(file);

    // on x=-20..26,y=-36..17,z=-47..7
    let re = Regex::new(r"(on|off) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)").unwrap();
    let mut res = Vec::new();
    
    for line_res in reader.lines() {
        if let Ok(line) = line_res {
            let caps = re.captures(line.trim()).unwrap();
            
            let x1 = caps[2].parse::<i64>().unwrap();
            let x2 = caps[3].parse::<i64>().unwrap();
            let y1 = caps[4].parse::<i64>().unwrap();
            let y2 = caps[5].parse::<i64>().unwrap();
            let z1 = caps[6].parse::<i64>().unwrap();
            let z2 = caps[7].parse::<i64>().unwrap();
            
            res.push(Cuboid {
                sign: &caps[1] == "on",
                x1: if x1 <= x2 { x1 } else { x2 },
                x2: if x1 <= x2 { x2 } else { x1 },
                y1: if y1 <= y2 { y1 } else { y2 },
                y2: if y1 <= y2 { y2 } else { y1 },
                z1: if z1 <= z2 { z1 } else { z2 },
                z2: if z1 <= z2 { z2 } else { z1 },
            });
        }
    }

    res
}

fn get_intersecting_cuboid(l: &Cuboid, r: &Cuboid) -> Cuboid {
    Cuboid { 
        x1: if r.x1 >= l.x1 { r.x1 } else { l.x1 },
        x2: if r.x2 <= l.x2 { r.x2 } else { l.x2 },
        y1: if r.y1 >= l.y1 { r.y1 } else { l.y1 },
        y2: if r.y2 <= l.y2 { r.y2 } else { l.y2 },
        z1: if r.z1 >= l.z1 { r.z1 } else { l.z1 },
        z2: if r.z2 <= l.z2 { r.z2 } else { l.z2 },
        sign: !r.sign
    }
}

fn intersect(l: &Cuboid, r: &Cuboid) -> bool {
    (l.x1 <= r.x2 && r.x1 <= l.x2) &&
    (l.y1 <= r.y2 && r.y1 <= l.y2) &&
    (l.z1 <= r.z2 && r.z1 <= l.z2)
}

fn get_intersecting_cuboids(cuboid: &Cuboid, res_cuboids: &Vec<Cuboid>) -> Vec<Cuboid> {
    let mut res = Vec::new();
    
    for r_cuboid in res_cuboids {
        if intersect(cuboid, r_cuboid) {
            let intersecting_cuboid = get_intersecting_cuboid(cuboid, r_cuboid);
            res.push(intersecting_cuboid);
        }
    }
    
    res
}

fn get_lit_cubes(cuboids: &Vec<Cuboid>) -> i64 {
    let mut res_cuboids = Vec::new();
    let mut total_sum = 0i64;

    for cuboid in cuboids {
        let mut intersecting_cuboids = get_intersecting_cuboids(cuboid, &res_cuboids);
        for i_cuboid in &intersecting_cuboids {
            total_sum += i_cuboid.volume();
        }
        res_cuboids.append(&mut intersecting_cuboids);
        
        if cuboid.sign {
            total_sum += cuboid.volume();
            res_cuboids.push(*cuboid);
        }
    }
    
    total_sum
}

fn main() {
    let cuboids = load_from_file("data.in");
    println!("{:}", get_lit_cubes(&cuboids));
} 

