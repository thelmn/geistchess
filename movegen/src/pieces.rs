use std::iter;

use crate::board::{BoardState, BitBoard, rank_bb, Direction};
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

    pub fn player_pieces_i(player: bool) -> usize {
        if player { 0 } else { 6 }
    }

    pub const STD_BITBOARDS: [BitBoard; STD_PIECECOUNT] = [
        STD_PAWNS_WHITE,        // White Pawns
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

    pub fn piece_char(&self) -> char {
        match self {
            Piece{ piece_type: PieceType::Pawn,   player: WHITE } => 'P',
            Piece{ piece_type: PieceType::Knight, player: WHITE } => 'N',
            Piece{ piece_type: PieceType::Bishop, player: WHITE } => 'B',
            Piece{ piece_type: PieceType::Rook,   player: WHITE } => 'R',
            Piece{ piece_type: PieceType::Queen,  player: WHITE } => 'Q',
            Piece{ piece_type: PieceType::King,   player: WHITE } => 'K',
            
            Piece{ piece_type: PieceType::Pawn,   player: BLACK } => 'p',
            Piece{ piece_type: PieceType::Knight, player: BLACK } => 'n',
            Piece{ piece_type: PieceType::Bishop, player: BLACK } => 'b',
            Piece{ piece_type: PieceType::Rook,   player: BLACK } => 'r',
            Piece{ piece_type: PieceType::Queen,  player: BLACK } => 'q',
            Piece{ piece_type: PieceType::King,   player: BLACK } => 'k',
            _ => '·',
        }
    }

    pub fn invalid() -> Piece {
        Piece{ piece_type: PieceType::Invalid, player: WHITE }
    }
    
    /// Return the piece attack mask and the the ray that checks the king, if king is in check
    pub fn attack_check_mask(&self, piece_mask: &BitBoard, empty: &BitBoard, king_mask: &BitBoard) -> (BitBoard, BitBoard) {
        let forward = self.player;
        match self.piece_type {
            PieceType::Pawn => {
                let mut check_mask = 0;
                // forward left capture
                let dir = if forward { Direction::NW } else { Direction::SE };
                let cp_l_dest = utils::slide(*piece_mask, 1, &dir);
                if cp_l_dest & king_mask > 0 {
                    check_mask |= utils::slide(*king_mask, 1, &dir.opp())
                }
                // forward right capture
                let dir = if forward { Direction::NE } else { Direction::SW };
                let cp_r_dest = utils::slide(*piece_mask, 1, &dir);
                if cp_l_dest & king_mask > 0 {
                    check_mask |= utils::slide(*king_mask, 1, &dir.opp())
                }
                (cp_l_dest | cp_r_dest, check_mask)
            },
            PieceType::Knight => {
                let mut a_mask = 0;
                let mut check_mask = 0;
                for knight_pos in BitPositions(*piece_mask) {
                    let attack_mask = utils::knight_attack(knight_pos);
                    a_mask |= attack_mask;
                    if attack_mask & king_mask > 0 {
                        for king_pos in BitPositions(*king_mask) {
                            check_mask |= utils::ray(king_pos, knight_pos);
                        }
                    }
                }
                (a_mask, check_mask)
            },
            PieceType::Bishop => {
                let mut a_mask = 0;
                let mut check_mask = 0;
                for bishop_pos in BitPositions(*piece_mask) {
                    let attack_mask = utils::bishop_attack(bishop_pos, !empty);
                    a_mask |= attack_mask;
                    if attack_mask & king_mask > 0 {
                        for king_pos in BitPositions(*king_mask) {
                            check_mask |= utils::ray(king_pos, bishop_pos);
                        }
                    }
                }
                (a_mask, check_mask)
            },
            PieceType::Rook => {
                let mut a_mask = 0;
                let mut check_mask = 0;
                for rook_pos in BitPositions(*piece_mask) {
                    let attack_mask = utils::rook_attack(rook_pos, !empty);
                    a_mask |= attack_mask;
                    if attack_mask & king_mask > 0 {
                        for king_pos in BitPositions(*king_mask) {
                            check_mask |= utils::ray(king_pos, rook_pos);
                        }
                    }
                }
                (a_mask, check_mask)
            },
            PieceType::Queen => {
                let mut a_mask = 0;
                let mut check_mask = 0;
                for queen_pos in BitPositions(*piece_mask) {
                    let attack_mask = utils::bishop_attack(queen_pos, !empty);
                    let attack_mask = attack_mask ^ utils::rook_attack(queen_pos, !empty);
                    a_mask |= attack_mask;
                    if attack_mask & king_mask > 0 {
                        for king_pos in BitPositions(*king_mask) {
                            check_mask |= utils::ray(king_pos, queen_pos);
                        }
                    }
                }
                (a_mask, check_mask)
            },
            PieceType::King => {
                let mut a_mask = 0;
                for king_pos in BitPositions(*piece_mask) {
                    a_mask |= utils::king_attack(king_pos);    
                }
                (a_mask, 0)
            },
            PieceType::Invalid => {(0, 0)},
        }
    }
    
    /// Finds legal (almost) moves for this piece. Returns the piece(s) attack mask
    /// Only the corner case of enpassant capture leading to a check along the 4th rank is illegal
    pub fn move_list(&self, piece_mask: &BitBoard, board_state: &BoardState, move_list: &mut MoveList) {
        let board = board_state.board;

        let empty = board.empty_mask();
        let forward = self.player;

        let _player_mask = board.player_mask(forward);
        let oppnt_mask = board.player_mask(!forward);
        
        
        match self.piece_type {
            PieceType::Pawn => {

                // Clear pinned pawns
                // Loop through pinned pawns, add each pawn's moves separately with the pin ray as valid_mask.
                // Probably not efficient
                let pinned_pawns = piece_mask & board_state.pinned_mask;
                let piece_mask = piece_mask & !board_state.pinned_mask;

                for pawn_pos in BitPositions(pinned_pawns) {
                    if let Some(ray) = board_state.pinned_pieces.get(&pawn_pos) {
                        let valid_mask = if ray == &0 { &utils::ONES } else { ray };
                        pawn_moves(forward, &piece_mask, valid_mask, &empty, &oppnt_mask, &board.enp_target, self, move_list);
                    }
                }
                // use the check mask as the valid mask
                let valid_mask = if board_state.opp_check_mask == &0 { &utils::ONES } else { board_state.opp_check_mask };
                pawn_moves(forward, &piece_mask, valid_mask, &empty, &oppnt_mask, &board.enp_target, self, move_list);
            },
            PieceType::Knight => {
                for knight_pos in BitPositions(*piece_mask) {
                    let mut attack_mask = utils::knight_attack(knight_pos);
                    // Masking with the ray is unnecessary. Knights cant move along rays
                    if let Some(ray) = board_state.pinned_pieces.get(&knight_pos) {
                        attack_mask &= ray;
                    }
                    if *(board_state.opp_check_mask) > 0 {
                        attack_mask &= board_state.opp_check_mask;
                    }
                    let ncp_dest = attack_mask & empty;
                    let cp_dest = attack_mask & oppnt_mask;

                    let meta = MoveMeta::Quiet;
                    move_list.push_from_forpiece( self, &meta, iter::repeat(knight_pos), BitPositions(ncp_dest) );
                    let meta = MoveMeta::Capture;
                    move_list.push_from_forpiece( self, &meta, iter::repeat(knight_pos), BitPositions(cp_dest) );
                }
            },
            PieceType::Bishop => {
                for bishop_pos in BitPositions(*piece_mask) {
                    let mut attack_mask = utils::bishop_attack(bishop_pos, !empty);
                    // if piece is pinned, only move along the pin ray
                    if let Some(ray) = board_state.pinned_pieces.get(&bishop_pos) {
                        attack_mask &= ray;
                    }
                    // if in check only moves that block the check are valid
                    if *(board_state.opp_check_mask) > 0 {
                        attack_mask &= board_state.opp_check_mask;
                    }
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
                    let mut attack_mask = utils::rook_attack(rook_pos, !empty);
                    if let Some(ray) = board_state.pinned_pieces.get(&rook_pos) {
                        attack_mask &= ray;
                    }
                    if *(board_state.opp_check_mask) > 0 {
                        attack_mask &= board_state.opp_check_mask;
                    }
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
                    let attack_mask = utils::bishop_attack(queen_pos, !empty);
                    let mut attack_mask = attack_mask ^ utils::rook_attack(queen_pos, !empty);
                    if let Some(ray) = board_state.pinned_pieces.get(&queen_pos) {
                        attack_mask &= ray;
                    }
                    if *(board_state.opp_check_mask) > 0 {
                        attack_mask &= board_state.opp_check_mask;
                    }
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
                    let attack_mask = utils::king_attack(king_pos);
                    // clear attacked squares
                    let attack_mask = attack_mask & !board_state.opp_attack_mask;

                    let ncp_dest = attack_mask & empty;
                    let cp_dest = attack_mask & oppnt_mask;

                    // can only castle while not in check
                    if *(board_state.opp_check_mask) == 0 {
                        // check short castle rights
                        if board.castle_rights(forward, true) {
                            // check that opp does not attack the squares the king will travel over
                            if (board_state.opp_attack_mask & utils::castle_travel_squares(forward, true) == 0) &&
                                // and the squares between are not occupied
                                (!empty & utils::castle_empty_squares(forward, true) == 0) {
                                let meta = MoveMeta::Castle{ is_short: true };
                                move_list.push(Move::new(self, &meta, king_pos, 0));
                            }
                        }
                        // check long castle rights
                        if board.castle_rights(forward, false) {
                            if (board_state.opp_attack_mask & utils::castle_travel_squares(forward, false) == 0) &&
                                (!empty & utils::castle_empty_squares(forward, false) == 0) {
                                let meta = MoveMeta::Castle{ is_short: false };
                                move_list.push(Move::new(self, &meta, king_pos, 0));
                            }
                        }
                    }                    
    
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

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.piece_char())
    }
}

fn pawn_moves(
        forward: bool, 
        piece_mask: &BitBoard, 
        valid_mask: &BitBoard, 
        empty: &BitBoard, 
        opp_mask: &BitBoard,
        enp_target: &u8,
        self_: &Piece,
        move_list: &mut MoveList
    ) {

    let rank7 = if forward { rank_bb::SEVEN } else { rank_bb::TWO };
    let rank8 = if forward { rank_bb::EIGHT } else { rank_bb::ONE };
    
    // direction is reversed since we shift the dest(empty/opponent) squares towards our pawns
    let mut dir = if forward { Direction::S } else { Direction::N };
    // single pawn push
    // valid mask can be the combined pin rays or the check ray
    let pp1 = utils::slide(empty & valid_mask, 1, &dir) & piece_mask;
    // with promotion
    let pp_promo = pp1 & rank7;
    
    // double pawn push
    let mut pp2 = if forward { rank_bb::FOUR } else { rank_bb::FIVE };
    pp2 = utils::slide(pp2 & empty & valid_mask, 1, &dir) & empty;
    pp2 = utils::slide(pp2, 1, &dir) & piece_mask;
    
    // the piece destinations
    dir = if forward { Direction::N } else { Direction::S };
    let pp1_dest = utils::slide(pp1, 1, &dir);
    let pp_promo_dest = pp1_dest & rank8;

    let pp2_dest = utils::slide(pp2, 2, &dir);

    // normal capture

    // forward left capture
    dir = if forward { Direction::SE } else { Direction::NW };
    let cp_l = utils::slide(opp_mask & valid_mask, 1, &dir) & piece_mask;
    // with promotion
    let cp_l_promo = cp_l & rank7;
    // dest
    dir = if forward { Direction::NW } else { Direction::SE };
    let cp_l_dest = utils::slide(cp_l, 1, &dir);
    let cp_l_promo_dest = cp_l_dest & rank8;

    // forward right capture
    dir = if forward { Direction::SW } else { Direction::NE };
    let cp_r = utils::slide(opp_mask & valid_mask, 1, &dir) & piece_mask;
    // with promotion. the promo sqs from cp_r are cleared at gen moves
    let cp_r_promo = cp_r & rank7;
    // dest
    dir = if forward { Direction::NE } else { Direction::SW };
    let cp_r_dest = utils::slide(cp_r, 1, &dir);
    let cp_r_promo_dest = cp_r_dest & rank8;

    // enpassant capture
    let mut cp_enp = 0;
    let mut cp_enp_dest = 0;
    if *enp_target != 0 &&
        *valid_mask == utils::ONES  // can't enpassant capture to block a check or when pinned
        {
            cp_enp_dest = if forward { enp_target+8 } else { enp_target-8 };
            let dest_mask = utils::pos_mask(cp_enp_dest);
            cp_enp = if forward { 
                utils::slide(dest_mask, 1, &Direction::SW) |
                utils::slide(dest_mask, 1, &Direction::SE)
             } else {
                utils::slide(dest_mask, 1, &Direction::NW) |
                utils::slide(dest_mask, 1, &Direction::NE)
             } & piece_mask;
        }


    let meta = MoveMeta::Quiet;
    move_list.push_from_forpiece( self_, &meta, BitPositions(pp1 & !rank7), BitPositions(pp1_dest & !rank8) );
    move_list.push_from_forpiece( self_, &meta, BitPositions(pp2), BitPositions(pp2_dest) );
    let meta = MoveMeta::Capture;
    move_list.push_from_forpiece( self_, &meta, BitPositions(cp_l & !rank7), BitPositions(cp_l_dest & !rank8) );
    move_list.push_from_forpiece( self_, &meta, BitPositions(cp_r & !rank7), BitPositions(cp_r_dest & !rank8) );
    let meta = MoveMeta::Enpassant;
    move_list.push_from_forpiece( self_, &meta, BitPositions(cp_enp), iter::repeat(cp_enp_dest) );
    // each possible target piecetype for promo is a separate move
    for promo_to in &PROMO_TARGETS {
        let meta = MoveMeta::Promotion{ is_capture: false,  piece_type: *promo_to };
        move_list.push_from_forpiece( self_, &meta, BitPositions(pp_promo), BitPositions(pp_promo_dest) );
        let meta = MoveMeta::Promotion{ is_capture: true,  piece_type: *promo_to };
        move_list.push_from_forpiece( self_, &meta, BitPositions(cp_l_promo), BitPositions(cp_l_promo_dest) );
        move_list.push_from_forpiece( self_, &meta, BitPositions(cp_r_promo), BitPositions(cp_r_promo_dest) );
    }
}
