use std::collections::VecDeque;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use regex::Regex;

type Point = [i64; 3];

#[derive(Debug, Clone)]
struct Rotation {
    p: Point,
    s: Point,
}

#[derive(Debug, Clone)]
struct Scanner {
    beacons: Vec<Point>,
}

#[derive(Debug, Clone)]
struct ScanDep {
    parent: usize,
    offset: Point,
}

fn load_from_file(file_path: &str) -> Vec<Scanner> {
    let file = File::open(file_path).expect(std::format!("File not found: {:}", file_path).as_str());
    let reader = BufReader::new(file);

    let re = Regex::new(r"(-?\d+),(-?\d+),(-?\d+)").unwrap();
    let mut result: Vec<Scanner> = Vec::new();
    for line in reader.lines() {
        let ll = line.unwrap();
        let l = ll.trim();
        if l.len() == 0 {
            continue;
        }
        
        if l.contains('s') {
            let mut beacons = Vec::new();
            beacons.reserve(26);
            result.push(Scanner { beacons });
            continue;
        }
        

        let caps = re.captures(&l).unwrap();
        let last_i = result.len() - 1;
        result[last_i].beacons.push([
            caps[1].parse::<i64>().unwrap(),
            caps[2].parse::<i64>().unwrap(),
            caps[3].parse::<i64>().unwrap(),
        ]);
    }

    result
}

fn has_positive_determinant(rot: &Rotation) -> bool {
    let mut m = vec![0; 9];
    m[0 + rot.p[0] as usize] = rot.s[0];
    m[3 + rot.p[1] as usize] = rot.s[1];
    m[6 + rot.p[2] as usize] = rot.s[2];

    // https://www.mathsisfun.com/algebra/matrix-determinant.html
    (
          m[0] * (m[4] * m[8] - m[5] * m[7]) 
        - m[1] * (m[3] * m[8] - m[5] * m[6])
        + m[2] * (m[3] * m[7] - m[4] * m[6])
    ) > 0
}

fn get_rotations() -> Vec<Rotation> {
    let mut res = Vec::new();
    
    for s_0 in [ 1, -1 ] {
        for s_1 in [ 1, -1 ] {
            for s_2 in [ 1, -1 ] {
                for p in [(0,1,2), (0,2,1), (1,0,2), (1,2,0), (2,0,1), (2,1,0)] {
                    res.push(Rotation {
                        p: [p.0, p.1, p.2],
                        s: [s_0, s_1, s_2]
                    });
                }
            }
        }
    }

    // optimization - keep only valid 90 degree rotations (48 rots -> 24 rots)
    res.iter().filter(|rot| has_positive_determinant(rot)).cloned().collect::<Vec<Rotation>>()
}

fn rotate(beacons: &Vec<Point>,  rot: &Rotation, out: &mut Vec<Point>) {
    for i in 0..beacons.len() {
        let beacon = beacons[i];
        let out_beacon = &mut out[i];
        out_beacon[0] = beacon[rot.p[0].abs() as usize] * rot.s[0];
        out_beacon[1] = beacon[rot.p[1].abs() as usize] * rot.s[1];
        out_beacon[2] = beacon[rot.p[2].abs() as usize] * rot.s[2];
    }
}

fn get_scanner_offset(beacons_i: &Vec<Point>, beacons_j: &Vec<Point>) -> Option<Point> {
    for beacon_i in beacons_i {
        for beacon_j in beacons_j {
            let bd: Point = [
                beacon_i[0] - beacon_j[0],
                beacon_i[1] - beacon_j[1],
                beacon_i[2] - beacon_j[2],
            ];
            
            let mut count = 0;
            for beacon_j_2 in beacons_j {
                let beacon = [
                    beacon_j_2[0] + bd[0],
                    beacon_j_2[1] + bd[1],
                    beacon_j_2[2] + bd[2],
                ];
                
                count += beacons_i.contains(&beacon) as i32;
            }

            if count >= 12 {
                return Some(bd);
            }
        }
    }
    
    None
}

fn add_points(left: &Point, right: &Point) -> Point {
    [
        left[0] + right[0],
        left[1] + right[1],
        left[2] + right[2],
    ]
}

fn get_scanner_dependencies_and_orient_scanners(scanners: &mut Vec<Scanner>) -> Vec<ScanDep> {
    let rotations = get_rotations();
    
    let mut scan_deps: Vec<ScanDep> = vec![ScanDep { parent: 0, offset: [0; 3] }; scanners.len()];
    let mut found = vec![false; scanners.len()];
    let mut indexes = VecDeque::new();
    
    indexes.push_back(0);
    found[0] = true;
    
    // map out contigous scanner dependencies and save the relative scanner-to-scanner_0 offsets 
    while let Some(i) = indexes.pop_front() {
        for j in 0..scanners.len() {
            if found[j] {
                continue;
            }
            
            let beacons_j = &scanners[j].beacons;
            let mut beacons_buf = beacons_j.clone();
            
            for rot in &rotations {
                rotate(beacons_j, &rot, &mut beacons_buf);

                if let Some(offset) = get_scanner_offset(&scanners[i].beacons, &beacons_buf) {
                    // println!("scanners matching: {:}, {:}, {:?}", i, j, offset);
                    scanners[j].beacons = beacons_buf;
                    scan_deps[j] = ScanDep {
                        parent: i,
                        offset: add_points(&scan_deps[i].offset, &offset)
                    };
                    indexes.push_back(j);
                    found[j] = true;
                    break;
                }
            }
        }
    }
    
    scan_deps
}

fn translate_beacons(scanners: &mut Vec<Scanner>, scan_deps: &Vec<ScanDep>) {
    // move all scanners relative to their offsets
    for i in 0..scanners.len() {
        let scan_offset = scan_deps[i].offset;
        for beacon in scanners[i].beacons.iter_mut() {
            beacon[0] += scan_offset[0];
            beacon[1] += scan_offset[1];
            beacon[2] += scan_offset[2];
        }
    }
}

fn get_max_distance(scan_deps: &Vec<ScanDep>) -> i64 {
    let mut max_dist = 0; 
    for sd1 in scan_deps {
        for sd2 in scan_deps {
            let dist = get_dist(&sd1.offset, &sd2.offset);
            if dist > max_dist {
                max_dist = dist;
            }
        }
    }
    max_dist
}

fn get_dist(left: &Point, right: &Point) -> i64 {
    (left[0] - right[0]).abs() + 
    (left[1] - right[1]).abs() + 
    (left[2] - right[2]).abs()
}

fn main() {
    let mut scanners = load_from_file("data.in");
    let scan_deps = get_scanner_dependencies_and_orient_scanners(&mut scanners);
    translate_beacons(&mut scanners, &scan_deps);
    println!("{:}", get_max_distance(&scan_deps));
} 
