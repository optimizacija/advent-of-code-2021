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

#[derive(Copy, Clone, Eq, PartialEq)]
struct PqItem {
    row: i32,
    col: i32,
    cost: u32,
}

impl Ord for PqItem {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.cost {
            b if b > other.cost => Ordering::Less,
            b if b < other.cost => Ordering::Greater,
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
    pq.push(PqItem { row: 0, col: 0, cost: 0 });

    while let Some(PqItem { row, col, cost }) = pq.pop() {
        if goal_reached(map, row, col) {
            return Some(cost);
        }
        visited[get_idx(map, row, col) as usize] = true;

        for (r, c) in [(row, col + 1),(row + 1, col), (row, col - 1), (row - 1, col)] {
            if is_in_bounds(map, r, c) && !visited[get_idx(map, r, c) as usize] {
                pq.push(PqItem { row: r, col: c, cost: cost + get_cost(map, r, c) });
            }
        }
    }

    None
}

fn main() {
    let map = load_from_file("data.in");
    println!("{:}", search(&map).unwrap());
}
