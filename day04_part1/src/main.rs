use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

type Board = Vec<i32>;

#[derive(Debug)]
struct BingoInput {
    draw_order: Vec<i32>,
    boards: Vec<Board>,
}

fn load_from_file(file_path: &str) -> BingoInput {
    let file = File::open(file_path).expect("file wasn't found.");
    let mut reader = BufReader::new(file);

    // read first line
    let mut buf = String::new();
    reader.read_line(&mut buf).unwrap();
    let draw_order: Vec<i32> = buf.split(',').map(|val| val.trim().parse::<i32>().unwrap()).collect();
    buf.clear();

    // read bingo boards
    let mut boards: Vec<Board> = vec![];
    let mut buf_board: Board = vec![0; 25];
    while let Ok(res) = reader.read_line(&mut buf) {
        if res == 0 { 
            break
        }
        
        if buf.trim().len() == 0 {
            buf.clear();
            for i in 0..5 {
                reader.read_line(&mut buf).unwrap();
                let numbers: Vec<i32> = buf.trim().split_whitespace().map(|val| val.parse::<i32>().unwrap()).collect();
                buf.clear();
                for j in 0..5 {
                    buf_board[i * 5 + j] = numbers[j];
                }
                
            }
            boards.push(buf_board.clone());
        }
    }
    
    BingoInput { draw_order, boards }
}

fn board_has_bingo(board: &Board) -> bool {
    // check rows
    for i in 0..5 {
        let mut row_has_bingo = true;
        
        for j in 0..5 {
            if board[i * 5 + j] != -1 {
                row_has_bingo = false;
                break;
            }
        }

        if row_has_bingo {
            return true;
        }
    }
 
    // check cols
    for j in 0..5 {
        let mut col_has_bingo = true;
        
        for i in 0..5 {
            if board[i * 5 + j] != -1 {
                col_has_bingo = false;
                break;
            }
        }

        if col_has_bingo {
            return true;
        }
    }
    
    false
}

fn get_non_bingo_sum(board: &Board) -> i32 {
    board.iter().filter(|val| **val != -1).sum()
}

fn main() {
    let mut board_input = load_from_file("data.in");
 
    // draw a value
    for draw_val in &board_input.draw_order {
        // match value on all boards
        for board in board_input.boards.iter_mut() {
            for j in 0..board.len() {
                if board[j] == *draw_val {
                    board[j] = -1;
                }
            }
        }
        
        // find board which has a bingo
        for board in board_input.boards.iter() {
            if board_has_bingo(board) {
                let non_bingo_sum = get_non_bingo_sum(board);
                println!("{:}", draw_val * non_bingo_sum);
                return;
            }
        }
    }

    panic!("Failed to find bingo");
}
