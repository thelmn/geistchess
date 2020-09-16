use std::env;
use std::fs::OpenOptions;
use std::io::{BufReader, BufRead};
use movegen::board::Board;
use movegen::graph::{boards_2_graphs, save_graphs};

const DATASET_BATCH: usize = 1000;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        let filename = &args[1];
        read_dataset_file(filename);
    } else {
        eprintln!("Not enough arguments. Expected fends2graphs <Dataset file>")
    }
}

fn read_dataset_file(filename: &str) {
    match OpenOptions::new().read(true).open(filename) {
        Ok(file_in) =>  {
            let mut boards = [Board::empty(); DATASET_BATCH];
            let mut next_players = [true; DATASET_BATCH];
            let mut scores = [0; DATASET_BATCH];
            let mut results = [0; DATASET_BATCH];
            
            let mut line_state = 0;
            let mut board: Board = Board::empty();
            let mut next_player: bool = true;
            let mut score: isize = 0;
            let mut result: i8 = 0;

            let mut ds_count = 0;
            let mut i = 0;

            for line in BufReader::new(file_in).lines() {
                if let Ok(line) = line {
                    if line.starts_with("fen ") && line_state == 0 {
                        let fen_str = line.trim_start_matches("fen ");
                        if let Ok(_board) = Board::from_fenstr(fen_str) {
                            println!("board {}: {}", i, fen_str);
                            board = _board;
                            next_player = board.player;
                            line_state = 1;
                        }
                    } else if line.starts_with("score ") && line_state == 1 {
                        let score_str = line.trim_start_matches("score ");
                        if let Ok(_score) = score_str.parse::<isize>() {
                            score = _score;
                            line_state = 2;
                        }
                    } else if line.starts_with("result ") && line_state == 2 {
                        let result_str = line.trim_start_matches("result ");
                        if let Ok(_result) = result_str.parse::<i8>() {
                            result = _result;
                            line_state = 3;
                        }
                    } else if line.starts_with("e") && line_state == 3 {
                        boards[i] = board;
                        next_players[i] = next_player;
                        scores[i] = score;
                        results[i] = result;
                        line_state = 0;
                        i += 1;
                    }
                }
                if i == DATASET_BATCH {
                    let graphs = boards_2_graphs(DATASET_BATCH, boards.iter());
                    let ds_batch_name = format!("{}{}.h5", filename, ds_count);
                    if let Err(e) = save_graphs(
                        ds_batch_name.as_str(),
                        DATASET_BATCH, graphs, &next_players, &scores, &results) {
                            eprintln!("failed to save graph ds {}: {}", ds_batch_name, e)
                        }
                    i = 0;
                    ds_count += 1;
                }
            }
        },
        Err(e) => {
            eprintln!("could not read file {}: {}", filename, e)
        }
    }
}