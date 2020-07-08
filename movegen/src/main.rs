#![allow(dead_code)]

mod board;
mod macros;
mod moves;
mod pieces;
mod utils;

use board::Board;
use moves::{Move, MoveList};

fn main() {
    println!("Hello, world!");
    let b = Board::standard();
    let mut move_list = MoveList::empty();
    let white = true;
    b.move_list(white, &Move::invalid(), &mut move_list);
    for m in &move_list {
        println!("{}", m);
    }
    println!("count: {}", move_list.count);
}
