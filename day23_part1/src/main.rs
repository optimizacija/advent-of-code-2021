use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use regex::Regex;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
enum Char {
    A,
    B,
    C,
    D
}

impl Char {
    fn cost(self: &Self) -> i64 {
        match self {
            Char::A => 1,
            Char::B => 10,
            Char::C => 100,
            Char::D => 1000,
        }
    }
    
    fn base_x(self: &Self) -> i64 {
        match self {
            Char::A => 2,
            Char::B => 4,
            Char::C => 6,
            Char::D => 8,
        }
    }
}


#[derive(Debug)]
struct Point {
    x: i64, 
    y: i64,
}


#[derive(Debug, Clone)]
struct BoardMove {
    x: i64, 
    y: i64,
    cost: i64,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct CharState {
    x: i64,
    y: i64,
    c: Char,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct BoardState {
    char_states: Vec<CharState>,
    cost: i64,
}


#[derive(Debug, Eq, PartialEq)]
enum TileType {
    Char(Char),
    Floor,
    ForbiddenFloor,
    Wall,
}

impl BoardState {
    fn solved(self: &Self) -> bool {
        for cs in &self.char_states {
            if !match cs.c {
                Char::A => cs.x == cs.c.base_x() && (cs.y == 1 || cs.y == 2),
                Char::B => cs.x == cs.c.base_x() && (cs.y == 1 || cs.y == 2),
                Char::C => cs.x == cs.c.base_x() && (cs.y == 1 || cs.y == 2),
                Char::D => cs.x == cs.c.base_x() && (cs.y == 1 || cs.y == 2),
            } {
                return false;
            }
        }
        
        true
    }
    
    fn is_solved(self: &Self, i: usize) -> bool {
        let cs = &self.char_states[i];
        match cs.c {
            Char::A => cs.x == cs.c.base_x() && (cs.y == 2 || (cs.y == 1 && self.get_tile_type(cs.x, 2) == TileType::Char(Char::A))),
            Char::B => cs.x == cs.c.base_x() && (cs.y == 2 || (cs.y == 1 && self.get_tile_type(cs.x, 2) == TileType::Char(Char::B))),
            Char::C => cs.x == cs.c.base_x() && (cs.y == 2 || (cs.y == 1 && self.get_tile_type(cs.x, 2) == TileType::Char(Char::C))),
            Char::D => cs.x == cs.c.base_x() && (cs.y == 2 || (cs.y == 1 && self.get_tile_type(cs.x, 2) == TileType::Char(Char::D))),
        }
    }
    
    fn get_tile_type(self: &Self, x: i64, y: i64) -> TileType {
        if (x < 0 || x > 10 || y < 0 || y > 2) || 
            ((y == 1 || y == 2) && (x <= 1 || x == 3 || x == 5 || x == 7 || x >= 9))
        {
            return TileType::Wall;
        }
        
        if y == 0 && (x == 2 || x == 4 || x == 6 || x == 8) {
            return TileType::ForbiddenFloor;
        }
        
        if let Some(char_state) = self.char_states.iter().find(|char_state| char_state.x == x && char_state.y == y) {
            return TileType::Char(char_state.c.clone());
        }
        
        TileType::Floor
    }

    fn is_in_hallway(self: &Self, i: usize) -> bool {
        self.char_states[i].y == 0
    }

    fn get_open_base_spot(self: &Self, i: usize) -> Option<Point> {
        let cs = &self.char_states[i];
        let bx = cs.c.base_x();
        
        if let Some(_) = self.char_states.iter().find(|cs2| cs2.x == bx && cs2.y == 1) {
            return None;
        }
        
        if let Some(_) = self.char_states.iter().find(|cs2| cs2.x == bx && cs2.y == 2) {
            return Some(Point {x: bx, y: 1});
        }
        
        Some(Point {x: bx, y: 2})
    }

    fn is_occupied(self: &Self, x: i64, y: i64) -> bool {
        match self.get_tile_type(x, y) {
            TileType::Floor | TileType::ForbiddenFloor => false,
            _ => true,
        }
    }
}

impl Ord for BoardState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for BoardState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.cost.partial_cmp(&self.cost)
    }
}

fn load_from_file(file_path: &str) -> Vec<CharState> {
    let file = File::open(file_path).expect(std::format!("File not found: {:}", file_path).as_str());
    let mut reader = BufReader::new(file);
    let mut buf = String::new();
    // irrelevant
    reader.read_line(&mut buf).unwrap();
    reader.read_line(&mut buf).unwrap();
    
    let mut res = Vec::new();
    
    let re = Regex::new(r".*(\w)#(\w)#(\w)#(\w).*").unwrap();
    for i in 1..=2 {
        buf = String::new();
        reader.read_line(&mut buf).unwrap();
        let caps = re.captures(&buf).unwrap();
        
        for j in 0..4 {
            res.push(CharState { 
                x: 2 + j * 2,
                y: i, 
                c: match caps[j as usize + 1].parse::<char>().unwrap() {
                    'A' => Char::A,
                    'B' => Char::B,
                    'C' => Char::C,
                    'D' => Char::D,
                    x => panic!("invalid char: {:}", x),
                },
            });
        }
    }
    
    res
}

fn find_possible_moves(board_state: &BoardState, i: usize) -> Vec<BoardMove> {
    if board_state.is_solved(i) {
        return Vec::new();
    }
   
    if board_state.is_in_hallway(i) {
        find_move_to_base(board_state, i)
    } else {
        find_hallway_moves(board_state, i)
    }
}

fn find_hallway_moves_recursive(board_state: &BoardState, i: usize, cm: BoardMove, dir: i64) -> Vec<BoardMove> {
    if board_state.is_occupied(cm.x, cm.y) {
        return Vec::new();
    }
    
    find_hallway_moves_recursive_no_check(board_state, i, cm, dir)
}

fn find_hallway_moves_recursive_no_check(board_state: &BoardState, i: usize, cm: BoardMove, dir: i64) -> Vec<BoardMove> {
    let cs = &board_state.char_states[i];
    if cm.y > 0 { // move to hallway
        return find_hallway_moves_recursive(board_state, i, BoardMove { x: cm.x, y: cm.y - 1 , cost: cm.cost + cs.c.cost()}, 0);
    } else {
        let mut curr_moves = match board_state.get_tile_type(cm.x, cm.y) {
            TileType::Floor => vec![cm.clone()],
            _ => Vec::new(),
        };
        
        if dir >= 0 {
            curr_moves.append(&mut find_hallway_moves_recursive(board_state, i, BoardMove { x: cm.x + 1, y: cm.y, cost: cm.cost + cs.c.cost() }, 1));
        } 
        if dir <= 0{
            curr_moves.append(&mut find_hallway_moves_recursive(board_state, i, BoardMove { x: cm.x - 1, y: cm.y, cost: cm.cost + cs.c.cost() }, -1));
        }
        curr_moves
    }
} 

fn find_hallway_moves(board_state: &BoardState, i: usize) -> Vec<BoardMove> {
    let cs = &board_state.char_states[i];
    find_hallway_moves_recursive_no_check(board_state, i,  BoardMove { x: cs.x, y: cs.y, cost: 0 }, 0)
}

fn find_move_to_base(board_state: &BoardState, i: usize) -> Vec<BoardMove> {
    if let Some(spot) = board_state.get_open_base_spot(i) {
        let cs = &board_state.char_states[i];
        let min_x = std::cmp::min(cs.x, spot.x);
        let max_x = std::cmp::max(cs.x, spot.x);
        
        for x in min_x..=max_x {
            if x == cs.x {
                continue;
            }
            
            if match board_state.get_tile_type(x, cs.y) {
                TileType::Floor | TileType::ForbiddenFloor => false,
                _ => true
            } {
                return Vec::new();
            }
        }
        
        let cost = (max_x - min_x + spot.y) * cs.c.cost();
        return vec![BoardMove {x: spot.x, y: spot.y, cost }];
    }
    
    Vec::new()
}

fn get_next_board_states(
    board_state: &BoardState,
    cache: &mut HashMap<Vec<CharState>, i64>
) -> Vec<BoardState> {
    let mut res = Vec::new();
    
    for i in 0..board_state.char_states.len() {
        let board_moves: Vec<BoardMove> = find_possible_moves(board_state, i);
        
        // create new board states from all the possible moves from current char state
        let mut temp = Vec::new();
        for board_move in &board_moves {
            let mut new_char_states = board_state.char_states.clone();
            new_char_states[i].x = board_move.x;
            new_char_states[i].y = board_move.y;
            
            temp.push(BoardState { 
                char_states: new_char_states,
                cost: board_state.cost + board_move.cost,
            });
        }
        
        // add new board states & board states which lower the cost
        let mut t_res = Vec::new();
        for bs in &temp {
            if let Some(&cost) = cache.get(&bs.char_states) {
                if bs.cost >= cost {
                    continue;
                }
            }

            cache.insert(bs.char_states.clone(), bs.cost);
            t_res.push(bs.clone());
        }

        res.append(&mut t_res);
    }
    
    res
}

fn organize_amphipods(char_states: &Vec<CharState>) -> Option<i64> {
    let mut cache = HashMap::new();
    cache.insert(char_states.to_vec(), 0);
    
    let mut pq = BinaryHeap::new();
    pq.push(BoardState {
        char_states: char_states.to_vec(),
        cost: 0,
    });
    
    while let Some(board_state) = pq.pop() {
        if let Some(&cost) = cache.get(&board_state.char_states) {
            if board_state.cost != cost {
                continue;
            }
        }
        
        if board_state.solved() {
            return Some(board_state.cost);
        }
        
        let next_states: Vec<BoardState> = get_next_board_states(&board_state, &mut cache);

        pq.extend(next_states);
    }
    
    None
}

fn main() {
    let char_states = load_from_file("data.in");
    println!("{:}", organize_amphipods(&char_states).unwrap());
} 

