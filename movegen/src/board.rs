use crate::pieces::{Piece, PieceType};
use crate::pieces::std_pieces::*;
use crate::moves::{MoveList, Move, MoveMeta};
use crate::utils;

#[repr(u64)]
pub enum RankBB {
    One     = 0xff,
    Two     = 0xff_00,
    Three   = 0xff_00_00,
    Four    = 0xff_00_00_00,
    Five    = 0xff_00_00_00_00,
    Six     = 0xff_00_00_00_00_00,
    Seven   = 0xff_00_00_00_00_00_00,
    Eight   = 0xff_00_00_00_00_00_00_00,
}

#[repr(u64)]
pub enum FileBB {
    A = 0x01_01_01_01_01_01_01_01,
    B = 0x02_02_02_02_02_02_02_02,
    C = 0x04_04_04_04_04_04_04_04,
    D = 0x08_08_08_08_08_08_08_08,
    E = 0x10_10_10_10_10_10_10_10,
    F = 0x20_20_20_20_20_20_20_20,
    G = 0x40_40_40_40_40_40_40_40,
    H = 0x80_80_80_80_80_80_80_80,
}

pub enum Direction { NW, N, NE, E, SE, S, SW, W, }

impl Direction {
    pub fn opp(&self) -> Direction {
        match self {
            &Direction::NW => Direction::SE,
            &Direction::N  => Direction::S,
            &Direction::NE => Direction::SW,
            &Direction::E  => Direction::W,
            &Direction::SE => Direction::NW,
            &Direction::S  => Direction::N,
            &Direction::SW => Direction::NE,
            &Direction::W  => Direction::E,
        }
    }
}

pub type BitBoard = u64;
pub type Square = u8;

pub struct Board<'a> {
    pub bitboards: [BitBoard; STD_PIECECOUNT],
    pub previous: Option<&'a Board<'a>>
}

impl<'a> Board<'a> {
    pub fn is_root(&self) -> bool {
        self.previous.is_none()
    }
    pub fn standard() -> Board<'a> {
        Board {
            bitboards: STD_BITBOARDS,
            previous: None
        }
    }
    pub fn move_list(&self, player: bool, movelist: &mut MoveList) {
        self.bitboards.iter()
                .enumerate()
                .filter(|(i, _)| get_piece(*i).player == player)
                .for_each(|(i, bb)| get_piece(i).move_list(bb, self, &Move::invalid(), movelist))
    }
    pub fn piece_mask(&self, piece: &Piece) -> BitBoard {
        *self.bitboards.get(get_piece_i(piece)).unwrap_or(&0)
    }
    pub fn empty_mask(&self) -> BitBoard {
        !self.bitboards.iter().fold(0, |acc, v| (acc | v) )
    }
    pub fn player_mask(&self, player: bool) -> BitBoard {
        self.bitboards.iter()
                .enumerate()
                .filter(|(i, _)| get_piece(*i).player == player)
                .fold(0, |acc, (_, bb)| (acc | bb))
    }
    pub fn make_move(&self, mov: Move) -> Board {
        // clear src and dest in all bbs
        let piece = mov.piece();
        let player = piece.player;
        let mut bitboards = self.bitboards.clone();

        let src_mask = utils::pos_mask(mov.src());
        let dest_mask = utils::pos_mask(mov.dest());

        match mov.move_meta() {
            MoveMeta::Castle{ is_short } => {
                bitboards[match_piece_i(PieceType::King, player)] ^= utils::king_castle(player, is_short);
                bitboards[match_piece_i(PieceType::Rook, player)] ^= utils::rook_castle(player, is_short);
            },
            MoveMeta::Enpassant => {
                bitboards[match_piece_i(PieceType::Pawn, !player)] ^= utils::pos_mask(if player { mov.dest()-8 } else { mov.dest()+8 });
                bitboards[get_piece_i(&piece)] ^= src_mask | dest_mask;
            },
            MoveMeta::Promotion{ is_capture: _, piece_type } => {
                // first clear all dest
                for bb in &mut bitboards {
                    *bb ^= dest_mask
                }
                bitboards[match_piece_i(piece_type, player)] ^= dest_mask;
                bitboards[get_piece_i(&piece)] ^= src_mask;
            },
            _ => {
                // first clear all dest
                for bb in &mut bitboards {
                    *bb ^= dest_mask
                }
                bitboards[get_piece_i(&piece)] ^= src_mask | dest_mask;
            }

        }
        Board { bitboards, previous: Some(self)}
    }
}