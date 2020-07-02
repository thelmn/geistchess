use std::iter;

use crate::board::{Board, BitBoard, RankBB, Direction};
use crate::moves::{Move, MoveMeta, BitPositions, MoveList};
use crate::utils;

pub const WHITE: bool = true;
pub const BLACK: bool = false;

pub mod std_pieces {
    use crate::board::BitBoard;
    use crate::pieces::{Piece, PieceType, WHITE, BLACK};
    
    pub const STD_PIECECOUNT: usize = 6*2;

    static WHITE_PAWN:   Piece = Piece{ piece_type: PieceType::Pawn,   player: WHITE };
    static WHITE_KNIGHT: Piece = Piece{ piece_type: PieceType::Knight, player: WHITE };
    static WHITE_BISHOP: Piece = Piece{ piece_type: PieceType::Bishop, player: WHITE };
    static WHITE_ROOK:   Piece = Piece{ piece_type: PieceType::Rook,   player: WHITE };
    static WHITE_QUEEN:  Piece = Piece{ piece_type: PieceType::Queen,  player: WHITE };
    static WHITE_KING:   Piece = Piece{ piece_type: PieceType::King,   player: WHITE };
    
    static BLACK_PAWN:   Piece = Piece{ piece_type: PieceType::Pawn,   player: BLACK };
    static BLACK_KNIGHT: Piece = Piece{ piece_type: PieceType::Knight, player: BLACK };
    static BLACK_BISHOP: Piece = Piece{ piece_type: PieceType::Bishop, player: BLACK };
    static BLACK_ROOK:   Piece = Piece{ piece_type: PieceType::Rook,   player: BLACK };
    static BLACK_QUEEN:  Piece = Piece{ piece_type: PieceType::Queen,  player: BLACK };
    static BLACK_KING:   Piece = Piece{ piece_type: PieceType::King,   player: BLACK };
        
    pub fn get_piece(i: usize) -> Piece {
        match i {
            0  => WHITE_PAWN,
            1  => WHITE_KNIGHT,
            2  => WHITE_BISHOP,
            3  => WHITE_ROOK,
            4  => WHITE_QUEEN,
            5  => WHITE_KING,
            6  => BLACK_PAWN,
            7  => BLACK_KNIGHT,
            8  => BLACK_BISHOP,
            9  => BLACK_ROOK,
            10 => BLACK_QUEEN,
            11 => BLACK_KING,
            _  => Piece::invalid()
        }
    }

    pub fn get_piece_i(piece: &Piece) -> usize {
        match piece {
            &Piece{ piece_type: PieceType::Pawn,   player: WHITE } => 0 ,
            &Piece{ piece_type: PieceType::Knight, player: WHITE } => 1 ,
            &Piece{ piece_type: PieceType::Bishop, player: WHITE } => 2 ,
            &Piece{ piece_type: PieceType::Rook,   player: WHITE } => 3 ,
            &Piece{ piece_type: PieceType::Queen,  player: WHITE } => 4 ,
            &Piece{ piece_type: PieceType::King,   player: WHITE } => 5 ,
            &Piece{ piece_type: PieceType::Pawn,   player: BLACK } => 6 ,
            &Piece{ piece_type: PieceType::Knight, player: BLACK } => 7 ,
            &Piece{ piece_type: PieceType::Bishop, player: BLACK } => 8 ,
            &Piece{ piece_type: PieceType::Rook,   player: BLACK } => 9 ,
            &Piece{ piece_type: PieceType::Queen,  player: BLACK } => 10,
            &Piece{ piece_type: PieceType::King,   player: BLACK } => 11,
            _ => 12
        }
    }

    pub fn match_piece_i(piece_type: PieceType, player: bool) -> usize {
        get_piece_i(&Piece{ piece_type, player })
    }

    pub const STD_BITBOARDS: [BitBoard; STD_PIECECOUNT] = [
        0,                      // White Pawns
        STD_KNIGHTS_WHITE,      // White Knights
        STD_BISHOPS_WHITE,      // White Bishops
        STD_ROOKS_WHITE,        // White Rooks
        STD_QUEENS_WHITE,       // White Queens
        STD_KINGS_WHITE,        // White Kings

        STD_PAWNS_BLACK,        // Black Pawns
        STD_KNIGHTS_BLACK,      // Black Knights
        STD_BISHOPS_BLACK,      // Black Bishops
        STD_ROOKS_BLACK,        // Black Rooks
        STD_QUEENS_BLACK,       // Black Queens
        STD_KINGS_BLACK,        // Black Kings
    ];
    
    const STD_PAWNS_WHITE: BitBoard = 0xff_00;
    const STD_KNIGHTS_WHITE: BitBoard = 0x42;
    const STD_BISHOPS_WHITE: BitBoard = 0x24;
    const STD_ROOKS_WHITE: BitBoard = 0x81;
    const STD_QUEENS_WHITE: BitBoard = 0x08;
    const STD_KINGS_WHITE: BitBoard = 0x10;
    
    const STD_PAWNS_BLACK: BitBoard = 0x00_ff_00_00_00_00_00_00;
    const STD_KNIGHTS_BLACK: BitBoard = 0x42_00_00_00_00_00_00_00;
    const STD_BISHOPS_BLACK: BitBoard = 0x24_00_00_00_00_00_00_00;
    const STD_ROOKS_BLACK: BitBoard = 0x81_00_00_00_00_00_00_00;
    const STD_QUEENS_BLACK: BitBoard = 0x08_00_00_00_00_00_00_00;
    const STD_KINGS_BLACK: BitBoard = 0x10_00_00_00_00_00_00_00;
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[repr(u8)]
pub enum PieceType {
    Invalid = 0,
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl PieceType {
    pub fn value(&self) -> u8 {
        *self as u8
    }
    pub fn from_value(v: u8) -> PieceType{
        match v {
            1 => PieceType::Pawn,       // 0b0001
            2 => PieceType::Knight,     // 0b0010
            3 => PieceType::Bishop,     // 0b0011
            4 => PieceType::Rook,       // 0b0100
            5 => PieceType::Queen,      // 0b0101
            6 => PieceType::King,       // 0b0110
            _ => PieceType::Invalid,    // 0b0000
        }
    }
    pub fn an(&self) -> &'static str {
        match self {
            PieceType::Pawn => "",
            PieceType::Knight => "N",
            PieceType::Bishop => "B",
            PieceType::Rook => "R",
            PieceType::Queen => "Q",
            PieceType::King => "K",
            PieceType::Invalid => "_",
        }
    }
}

const PROMO_TARGETS: [PieceType; 4] = [ PieceType::Knight, PieceType::Bishop, PieceType::Rook, PieceType::Queen ];

const PIECETYPE_BITS: u8 = 3;
const PIECETYPE_MASK: u8 = 0b0111;

// Piece(0b0000_1xxx) for white pieces. Piece(0b0000_0xxx) for black pieces
#[derive(Copy, Clone, PartialEq)]
pub struct Piece {
    pub piece_type: PieceType,
    pub player: bool 
}

impl Piece {    
    pub fn from_bits(piecebits: u8) -> Piece {
        Piece { 
            piece_type: PieceType::from_value(piecebits & PIECETYPE_MASK),
            player: piecebits > PIECETYPE_MASK
        }
    }

    pub fn to_bits(&self) -> u8 {
        ((self.player as u8) << PIECETYPE_BITS) | self.piece_type.value()
    }

    /// Algebraic notation   
    pub fn an(&self) -> &'static str {
        self.piece_type.an()
    } 

    pub fn invalid() -> Piece {
        Piece{ piece_type: PieceType::Invalid, player: WHITE }
    }
    
    pub fn move_list(&self, piece_mask: &BitBoard, board: &Board, prev_move: &Move, move_list: &mut MoveList) {

        let empty = board.empty_mask();
        let forward = self.player;

        let _player_mask = board.player_mask(forward);
        let oppnt_mask = board.player_mask(!forward);

        match self.piece_type {
            PieceType::Pawn => {

                let rank7 = if forward { RankBB::Seven } else { RankBB::Two } as u64;

                // direction is reversed since we shift the dest(empty/opponent) squares towards our pawns
                let mut dir = if forward { Direction::S } else { Direction::N };
                // single pawn push
                let pp1 = utils::slide(empty, 1, &dir) & piece_mask;
                // with promotion
                let pp_promo = pp1 & rank7;
                let pp1 = pp1 & !rank7; // clear rank7 of single pawn pushes
                
                // double pawn push
                let mut pp2 = if forward { RankBB::Four } else { RankBB::Five } as u64;
                pp2 = utils::slide(pp2 & empty, 1, &dir) & empty;
                pp2 = utils::slide(pp2, 1, &dir) & piece_mask;
                
                // the piece destinations
                dir = if forward { Direction::N } else { Direction::S };
                let pp1_dest = utils::slide(pp1, 1, &dir);
                let pp_promo_dest = utils::slide(pp_promo, 1, &dir);
                let pp2_dest = utils::slide(pp2, 2, &dir);

                // normal capture

                // forward left capture
                dir = if forward { Direction::SE } else { Direction::NW };
                let cp_l = utils::slide(oppnt_mask, 1, &dir) & piece_mask;
                // with promotion
                let cp_l_promo = cp_l & rank7;
                let cp_l = cp_l & !rank7; // clear rank7 of capture to left
                // dest
                dir = if forward { Direction::NW } else { Direction::SE };
                let cp_l_dest = utils::slide(cp_l, 1, &dir);
                let cp_l_promo_dest = utils::slide(cp_l_promo, 1, &dir);

                // forward right capture
                dir = if forward { Direction::SW } else { Direction::NE };
                let cp_r = utils::slide(oppnt_mask, 1, &dir) & piece_mask;
                // with promotion
                let cp_r_promo = cp_r & rank7;
                let cp_r = cp_r & !rank7; // clear rank7 of capture to right
                // dest
                dir = if forward { Direction::NE } else { Direction::SW };
                let cp_r_dest = utils::slide(cp_r, 1, &dir);
                let cp_r_promo_dest = utils::slide(cp_r_promo, 1, &dir);

                // enpassant capture
                let mut cp_enp = 0;
                let mut cp_enp_dest = 0;
                if prev_move.move_meta() == MoveMeta::Enpassant 
                    && prev_move.piece().player == !forward {
                        let dest = prev_move.dest();
                        cp_enp_dest = if forward { dest+8 } else { dest-8 };
                        let dest_mask = utils::pos_mask(dest);
                        cp_enp = if forward { 
                            utils::slide(dest_mask, 1, &Direction::SW) |
                            utils::slide(dest_mask, 1, &Direction::SE)
                         } else {
                            utils::slide(dest_mask, 1, &Direction::NW) |
                            utils::slide(dest_mask, 1, &Direction::NE)
                         } & piece_mask;
                    }

                let meta = MoveMeta::Quiet;
                move_list.push_from_forpiece( self, &meta, BitPositions(pp1), BitPositions(pp1_dest) );
                move_list.push_from_forpiece( self, &meta, BitPositions(pp2), BitPositions(pp2_dest) );
                let meta = MoveMeta::Capture;
                move_list.push_from_forpiece( self, &meta, BitPositions(cp_l), BitPositions(cp_l_dest) );
                move_list.push_from_forpiece( self, &meta, BitPositions(cp_r), BitPositions(cp_r_dest) );
                let meta = MoveMeta::Enpassant;
                move_list.push_from_forpiece( self, &meta, BitPositions(cp_enp), iter::repeat(cp_enp_dest) );
                // each possible target piecetype for promo is a separate move
                for promo_to in &PROMO_TARGETS {
                    let meta = MoveMeta::Promotion{ is_capture: false,  piece_type: *promo_to };
                    move_list.push_from_forpiece( self, &meta, BitPositions(pp_promo), BitPositions(pp_promo_dest) );
                    let meta = MoveMeta::Promotion{ is_capture: true,  piece_type: *promo_to };
                    move_list.push_from_forpiece( self, &meta, BitPositions(cp_l_promo), BitPositions(cp_l_promo_dest) );
                    move_list.push_from_forpiece( self, &meta, BitPositions(cp_r_promo), BitPositions(cp_r_promo_dest) );
                }
            },
            PieceType::Knight => {
                for king_pos in BitPositions(*piece_mask) {
                    let attack_mask = utils::patterns::KNIGHT_PATTERNS[king_pos as usize];
                    let ncp_dest = attack_mask & empty;
                    let cp_dest = attack_mask & oppnt_mask;

                    let meta = MoveMeta::Quiet;
                    move_list.push_from_forpiece( self, &meta, iter::repeat(king_pos), BitPositions(ncp_dest) );
                    let meta = MoveMeta::Capture;
                    move_list.push_from_forpiece( self, &meta, iter::repeat(king_pos), BitPositions(cp_dest) );
                }
            },
            PieceType::Bishop => {
                for bishop_pos in BitPositions(*piece_mask) {
                    let attack_mask = utils::magic::bishop_attack(bishop_pos, !empty);
                    let ncp_dest = attack_mask & empty;
                    let cp_dest = attack_mask & oppnt_mask;

                    let meta = MoveMeta::Quiet;
                    move_list.push_from_forpiece( self, &meta, iter::repeat(bishop_pos), BitPositions(ncp_dest) );
                    let meta = MoveMeta::Capture;
                    move_list.push_from_forpiece( self, &meta, iter::repeat(bishop_pos), BitPositions(cp_dest) );
                }
            },
            PieceType::Rook => {
                for rook_pos in BitPositions(*piece_mask) {
                    let attack_mask = utils::magic::rook_attack(rook_pos, !empty);
                    let ncp_dest = attack_mask & empty;
                    let cp_dest = attack_mask & oppnt_mask;

                    let meta = MoveMeta::Quiet;
                    move_list.push_from_forpiece( self, &meta, iter::repeat(rook_pos), BitPositions(ncp_dest) );
                    let meta = MoveMeta::Capture;
                    move_list.push_from_forpiece( self, &meta, iter::repeat(rook_pos), BitPositions(cp_dest) );
                }
            },
            PieceType::Queen => {
                for queen_pos in BitPositions(*piece_mask) {
                    let attack_mask = utils::magic::bishop_attack(queen_pos, !empty);
                    let attack_mask = attack_mask ^ utils::magic::rook_attack(queen_pos, !empty);
                    let ncp_dest = attack_mask & empty;
                    let cp_dest = attack_mask & oppnt_mask;

                    let meta = MoveMeta::Quiet;
                    move_list.push_from_forpiece( self, &meta, iter::repeat(queen_pos), BitPositions(ncp_dest) );
                    let meta = MoveMeta::Capture;
                    move_list.push_from_forpiece( self, &meta, iter::repeat(queen_pos), BitPositions(cp_dest) );
                }
            },
            PieceType::King => {
                for king_pos in BitPositions(*piece_mask) {
                    let attack_mask = utils::patterns::KING_PATTERNS[king_pos as usize];
                    let ncp_dest = attack_mask & empty;
                    let cp_dest = attack_mask & oppnt_mask;

                    // TODO: Castling. Pass a BoardState object?

                    let meta = MoveMeta::Quiet;
                    move_list.push_from_forpiece( self, &meta, iter::repeat(king_pos), BitPositions(ncp_dest) );
                    let meta = MoveMeta::Capture;
                    move_list.push_from_forpiece( self, &meta, iter::repeat(king_pos), BitPositions(cp_dest) );
                }
            },
            PieceType::Invalid => {},
        }
    }
}
