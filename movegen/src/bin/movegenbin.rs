// #![allow(dead_code)]
// extern crate movegen;

use movegen::board::Board;
use movegen::moves::MoveList;

fn main() {
    println!("Hello, world!");
    let b = Board::standard();
    let mut move_list = MoveList::new();
    let white = true;
    b.move_list(white, &mut move_list);
    for m in move_list.iter() {
        println!("{}", m);
    }
    println!("count: {}", move_list.iter().count());
}
