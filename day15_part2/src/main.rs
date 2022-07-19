use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

#[derive(Debug)]
struct Map {
    data: Vec<u32>,
    cols: i32,
    rows: i32,
}

fn load_from_file(file_path: &str) -> Map {
    let file = File::open(file_path).expect(std::format!("File not found: {:}", file_path).as_str());
    let reader = BufReader::new(file);
    
    let mut cols = 0usize;
    let data = reader.lines()
        .into_iter()
        .map(|line| {
            let line_str = line.unwrap();
            cols = line_str.len();
            line_str
                .chars()
                .map(|ch| ch.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .flatten()
        .collect::<Vec<u32>>();
        
    Map { rows: (data.len() / cols) as i32, data, cols: cols as i32 }
}

fn extend_map(map: &Map) -> Map {
    let o_row_count = 5usize;
    let o_col_count = 5usize;
    
    let o_cols = (map.rows * map.cols) as usize;
    let o_row_size = o_cols * o_col_count;
    let mut data = vec![0; o_row_size * o_row_count];
    
    let oi_cols = o_col_count * map.cols as usize;
    
    for or in 0..o_row_count {
        for oc in 0..o_col_count {
            for r in 0..(map.rows as usize) {
                for c in 0..(map.cols as usize) {
                    let inner_idx = r * (map.cols as usize) + c; 
                    let outer_idx = or * o_row_size + oc * (map.cols as usize) + r * oi_cols + c;
                    if or == 0 && oc == 0 {
                        data[outer_idx] = map.data[inner_idx];
                    } else if or == 0 { 
                        // check left
                        let outer_left_idx = (oc - 1) * (map.cols as usize) + r * oi_cols + c;
                        data[outer_idx] = std::cmp::max((data[outer_left_idx] + 1) % 10, 1);
                    } else { 
                        // check left
                        let outer_up_idx = (or - 1) * o_row_size + oc * (map.cols as usize) + r * oi_cols + c;
                        data[outer_idx] = std::cmp::max((data[outer_up_idx] + 1) % 10, 1);
                    }
                }
            }
        }
    }
    
    Map { data, cols: map.cols * o_col_count as i32, rows: map.rows * o_row_count as i32 }
}

fn is_in_bounds(map: &Map, row: i32, col: i32) -> bool {
    !(row < 0 || row >= map.rows || col < 0 || col >= map.cols)
}

fn get_idx(map: &Map, row: i32, col: i32) -> i32 {
    row * map.cols + col
}

fn get_cost(map: &Map, row: i32, col: i32) -> u32 {
    map.data[get_idx(map, row, col) as usize]
}

fn goal_reached(map: &Map, row: i32, col: i32) -> bool {
    row == map.rows - 1 && col == map.cols - 1 
}

fn heuristic(map: &Map, row: i32, col: i32) -> u32 {
    ((map.rows - 1 - row) + (map.cols - 1 - col)) as u32
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct PqItem {
    row: i32,
    col: i32,
    g_cost: u32,
    f_cost: u32,
}

impl Ord for PqItem {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.f_cost {
            b if b > other.f_cost => Ordering::Less,
            b if b < other.f_cost => Ordering::Greater,
            _ => Ordering::Equal,
        }
    }
}

impl PartialOrd for PqItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn search(map: &Map) -> Option<u32> {
    let mut pq = BinaryHeap::new();
    let mut visited = vec![false; map.data.len()];
    visited[0] = true;
    pq.push(PqItem { row: 0, col: 0, g_cost: 0, f_cost: 0 });

    while let Some(PqItem { row, col, g_cost, f_cost: _ }) = pq.pop() {
        if goal_reached(map, row, col) {
            return Some(g_cost);
        }
        visited[get_idx(map, row, col) as usize] = true;

        for (r, c) in [(row, col + 1),(row + 1, col), (row, col - 1), (row - 1, col)] {
            if is_in_bounds(map, r, c) && !visited[get_idx(map, r, c) as usize] {
                let new_g_cost = g_cost + get_cost(map, r, c);
                let new_f_cost = new_g_cost + heuristic(map, r, c);
                pq.push(PqItem { row: r, col: c, g_cost: new_g_cost, f_cost: new_f_cost });
            }
        }
    }

    None
}

fn main() {
    let map = load_from_file("data.in");
    println!("{:}", search(&extend_map(&map)).unwrap());
}
