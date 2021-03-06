use std::convert::TryInto;
use bitintr::{Blsr, Tzcnt};
use std::fmt;

use crate::pieces::{Piece, PieceType};
use crate::board::{Square, BitBoard};
use crate::utils;

pub const MOVE_ILLEGAL:     u8 = 0;
pub const MOVE_QUIET:       u8 = 1;
pub const MOVE_CAPTURE:     u8 = 2;
pub const MOVE_CASTLE_S:    u8 = 3;
pub const MOVE_CASTLE_L:    u8 = 4;
pub const MOVE_ENPASSANT:   u8 = 5;
pub const MOVE_PROMOTE_N:   u8 = 6;
pub const MOVE_PROMOTE_B:   u8 = 7;
pub const MOVE_PROMOTE_R:   u8 = 8;
pub const MOVE_PROMOTE_Q:   u8 = 9;
pub const MOVE_PROMOTE_N_X: u8 = 10;
pub const MOVE_PROMOTE_B_X: u8 = 11;
pub const MOVE_PROMOTE_R_X: u8 = 12;
pub const MOVE_PROMOTE_Q_X: u8 = 13;

#[derive(PartialEq, Debug)]
pub enum MoveMeta {
    Quiet,      // Quiet move that is not a castle
    Capture,    // Capture that is neither Enpassant or a promotion
    Castle { is_short: bool },
    Enpassant,
    Promotion {is_capture: bool, piece_type: PieceType},
    Illegal,
}

impl MoveMeta {
    fn to_bits(&self) -> u8 {
        match self {
            &MoveMeta::Quiet => MOVE_QUIET,
            &MoveMeta::Capture => MOVE_CAPTURE,
            &MoveMeta::Castle{ is_short } => if is_short {MOVE_CASTLE_S} else {MOVE_CASTLE_L},
            &MoveMeta::Enpassant => MOVE_ENPASSANT,
            &MoveMeta::Promotion{ is_capture: false, piece_type: PieceType::Knight } => MOVE_PROMOTE_N,
            &MoveMeta::Promotion{ is_capture: false, piece_type: PieceType::Bishop } => MOVE_PROMOTE_B,
            &MoveMeta::Promotion{ is_capture: false, piece_type: PieceType::Rook } => MOVE_PROMOTE_R,
            &MoveMeta::Promotion{ is_capture: false, piece_type: PieceType::Queen } => MOVE_PROMOTE_Q,
            &MoveMeta::Promotion{ is_capture: false, piece_type: _ } => MOVE_PROMOTE_Q,
            &MoveMeta::Promotion{ is_capture: true, piece_type: PieceType::Knight } => MOVE_PROMOTE_N_X,
            &MoveMeta::Promotion{ is_capture: true, piece_type: PieceType::Bishop } => MOVE_PROMOTE_B_X,
            &MoveMeta::Promotion{ is_capture: true, piece_type: PieceType::Rook } => MOVE_PROMOTE_R_X,
            &MoveMeta::Promotion{ is_capture: true, piece_type: PieceType::Queen } => MOVE_PROMOTE_Q_X,
            &MoveMeta::Promotion{ is_capture: true, piece_type: _ } => MOVE_PROMOTE_Q_X,
            &MoveMeta::Illegal => MOVE_ILLEGAL,
        }
    }
    fn from_bits(meta_bits: u8) -> Self {
        match meta_bits {
            MOVE_QUIET => MoveMeta::Quiet,
            MOVE_CAPTURE => MoveMeta::Capture,
            MOVE_CASTLE_S  => MoveMeta::Castle{ is_short: true },
            MOVE_CASTLE_L  => MoveMeta::Castle{ is_short: false },
            MOVE_ENPASSANT => MoveMeta::Enpassant,
            MOVE_PROMOTE_N => MoveMeta::Promotion{ is_capture: false, piece_type: PieceType::Knight },
            MOVE_PROMOTE_B => MoveMeta::Promotion{ is_capture: false, piece_type: PieceType::Bishop },
            MOVE_PROMOTE_R => MoveMeta::Promotion{ is_capture: false, piece_type: PieceType::Rook },
            MOVE_PROMOTE_Q => MoveMeta::Promotion{ is_capture: false, piece_type: PieceType::Queen },
            MOVE_PROMOTE_N_X => MoveMeta::Promotion{ is_capture: true, piece_type: PieceType::Knight },
            MOVE_PROMOTE_B_X => MoveMeta::Promotion{ is_capture: true, piece_type: PieceType::Bishop },
            MOVE_PROMOTE_R_X => MoveMeta::Promotion{ is_capture: true, piece_type: PieceType::Rook },
            MOVE_PROMOTE_Q_X => MoveMeta::Promotion{ is_capture: true, piece_type: PieceType::Queen },
            _ => MoveMeta::Illegal,
        }
    }
    pub fn is_capture(&self) -> bool {
        match self {
            &MoveMeta::Capture => true,
            &MoveMeta::Promotion{ is_capture, piece_type: _ } => is_capture,
            &MoveMeta::Enpassant => true,
            _ => false
        }
    }
}

pub const PIECE_MASK:       u8 = 0b1111_0000;
pub const PIECE_MASK_SHIFT: u8  = 4;
pub const MOVEMETA_MASK:    u8 = 0b0000_1111;

pub const SRC_MASK:         u16 = 0b01111111_00000000;
pub const SRC_MASK_SHIFT:   u16  = 8;
pub const DEST_MASK:        u16 = 0b00000000_01111111;

// Move(4bits piece _ 6bits source _ 6bits dest)
#[derive(Copy)]
pub struct Move{ meta: u8, srcdest: u16 }

impl Move {
    pub fn invalid() -> Self {
        Move{meta: MoveMeta::Illegal.to_bits(), srcdest: 0}
    }
    pub fn new(piece: &Piece, meta: &MoveMeta, src: Square, dest: Square) -> Self {
        let meta: u8 =  ((piece.to_bits() << PIECE_MASK_SHIFT) & PIECE_MASK) | 
                        (meta.to_bits() & MOVEMETA_MASK);
        let srcdest =   (((src as u16) << SRC_MASK_SHIFT) & SRC_MASK) |
                        ((dest as u16) & DEST_MASK);
        Move{meta, srcdest}
    }
    pub fn piece(&self) -> Piece {
        let piece_bits: u8 = (self.meta & PIECE_MASK) >> PIECE_MASK_SHIFT;
        Piece::from_bits(piece_bits)
    }
    pub fn move_meta(&self) -> MoveMeta {
        let meta_bits: u8 = self.meta & MOVEMETA_MASK;
        MoveMeta::from_bits(meta_bits)
    }
    pub fn src(&self) -> Square {
        let square: u8 = ((self.srcdest & SRC_MASK) >> SRC_MASK_SHIFT).try_into().unwrap();
        square
    }
    pub fn src_bb(&self) -> BitBoard {
        1 << self.src()
    }
    pub fn dest(&self) -> Square {
        let square: u8 = (self.srcdest & DEST_MASK).try_into().unwrap();
        square
    }
    pub fn dest_bb(&self) -> BitBoard {
        1 << self.dest()
    }
    pub fn is_invalid(&self) -> bool {
        self.meta == 0 && self.srcdest == 0
    }
}

impl Clone for Move {
    fn clone(&self) -> Self {
        *self
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let MoveMeta::Castle{ is_short } = self.move_meta() {
            return write!(f, "{}", if is_short { "O-O" } else {"O-O-O"});
        }
        let mut res = write!(
            f, 
            "{}{}{}{}", 
            self.piece().an(), 
            utils::file_rank_str(self.src()),
            if self.move_meta().is_capture() {"x"} else {""},
            utils::file_rank_str(self.dest())
        );
        if let MoveMeta::Promotion{ is_capture: _, piece_type } = self.move_meta() {
            res = write!(f, "={}", piece_type.an());
        }
        res
        // write!(f, "{}, {}, {}, {:#?}", self.piece().an(), utils::file_rank_str(self.src()), utils::file_rank_str(self.dest()), self.move_meta())
    }
}

pub const AVG_MOVE_COUNT: usize = 30;

pub struct MoveList {
    moves: Vec<Move>,
}

impl MoveList {
    pub fn new() -> Self {
        MoveList { 
            moves: Vec::with_capacity(AVG_MOVE_COUNT)
        }
    }
    pub fn clear(&mut self) {
        self.moves.clear()
    }
    pub fn push(&mut self, m: Move) {
        self.moves.push(m);
    }
    pub fn push_from<T: Iterator<Item = Move>>(&mut self, moves: T) {
        moves.for_each(|m| self.push(m))
    }
    
    pub fn push_from_forpiece<T, R>(&mut self, piece: &Piece, meta: &MoveMeta, srcs: T, dests: R)
    where T: Iterator<Item = u8>, R: Iterator<Item = u8> {
        srcs.zip(dests).for_each(|(src, dest)| self.push(Move::new(piece, meta, src, dest)))
    }
    
    pub fn get(&self) -> Move {
        self.moves[0]
    }

    pub fn iter(&self) -> impl Iterator<Item = &Move> {
        self.moves.iter()
    }
}

pub struct BitPositions(pub BitBoard);

impl Iterator for BitPositions {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 != 0 {
            let lsb_i = self.0.tzcnt();
            self.0 = self.0.blsr();
            return Some(lsb_i.try_into().unwrap())
        }
        None
    }
}

