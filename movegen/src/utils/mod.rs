
pub mod patterns;
pub mod magic;

use crate::board::{BitBoard, Direction};
use crate::pieces::{WHITE, BLACK};

static FILES: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

pub fn file(pos: u8) -> char {
    FILES[(pos%8) as usize]
}

pub fn file_rank(pos: u8) -> (u8, u8) {
    (pos%8, pos/8)
}

pub fn file_rank_str(pos: u8) -> String {
    format!("{}{}", FILES[(pos%8) as usize], (pos/8)+1)
}

const FILE_MASK_EAST: [BitBoard; 8] = [
    0x7f7f7f7f7f7f7f7f, 0x3f3f3f3f3f3f3f3f, 0x1f1f1f1f1f1f1f1f, 0x0f0f0f0f0f0f0f0f, 0x0707070707070707, 0x0303030303030303, 0x0101010101010101, 0x0
];

const FILE_MASK_WEST: [BitBoard; 8] = [
    0xfefefefefefefefe, 0xfcfcfcfcfcfcfcfc, 0xf8f8f8f8f8f8f8f8, 0xf0f0f0f0f0f0f0f0, 0xe0e0e0e0e0e0e0e0, 0xc0c0c0c0c0c0c0c0, 0x8080808080808080, 0x0
];

pub fn file_preshift_mask(bb: BitBoard, steps: usize, dir: &Direction) -> BitBoard {
    match dir {
        &Direction::NE | &Direction::E | &Direction::SE => bb & FILE_MASK_EAST[steps-1],
        &Direction::NW | &Direction::W | &Direction::SW => bb & FILE_MASK_WEST[steps-1],
        _ => bb
    }
}

// slide {steps} towards {Direction}
pub fn slide(bb: BitBoard, steps: u8, dir: &Direction) -> BitBoard {
    let bb = file_preshift_mask(bb, steps as usize, dir);
    match dir {
        // forward
        &Direction::NW => bb << 7*steps,
        &Direction::N => bb << 8*steps,
        &Direction::NE => bb << 9*steps,
        &Direction::E => bb << 1*steps,
        // backward
        &Direction::SE => bb >> 7*steps,
        &Direction::S => bb >> 8*steps,
        &Direction::SW => bb >> 9*steps,
        &Direction::W => bb >> 1*steps,
    }
}

pub fn pos_mask(pos: u8) -> BitBoard {
    1 << pos
}

pub fn king_castle(player: bool, is_short: bool) -> BitBoard {
    match (player, is_short) {
        (WHITE, true)  => 0x50,    // 0000 1010 src and dest set
        (WHITE, false) => 0x14,    // 0010 1000
        (BLACK, true)  => 0x50_00_00_00_00_00_00_00,
        (BLACK, false) => 0x14_00_00_00_00_00_00_00,
    }
}

pub fn rook_castle(player: bool, is_short: bool) -> BitBoard {
    match (player, is_short) {
        (WHITE, true)  => 0xA0,     // 0000 0101 src and dest set
        (WHITE, false) => 0x09,     // 1001 0000
        (BLACK, true)  => 0xA0_00_00_00_00_00_00_00,
        (BLACK, false) => 0x09_00_00_00_00_00_00_00,
    }
}

