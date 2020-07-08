use fnv::FnvHashMap;

use crate::pieces::{Piece, PieceType, WHITE, BLACK};
use crate::pieces::std_pieces::*;
use crate::moves::{MoveList, Move, MoveMeta, BitPositions};
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
    pub previous: Option<&'a Board<'a>>,
    pub castle_w_s: bool,
    pub castle_w_l: bool,
    pub castle_b_s: bool,
    pub castle_b_l: bool,
}

pub struct BoardState<'a> {
    pub board:           &'a Board<'a>,
    pub prev_move:       &'a Move,
    pub pinned_mask:     &'a BitBoard,
    pub pinned_pieces:   &'a FnvHashMap<u8, BitBoard>,
    pub opp_check_mask:  &'a BitBoard,
    pub opp_attack_mask: &'a BitBoard,
}

impl<'a> Board<'a> {
    pub fn is_root(&self) -> bool {
        self.previous.is_none()
    }
    pub fn standard() -> Board<'a> {
        Board {
            bitboards: STD_BITBOARDS,
            previous: None,
            castle_w_s: true,
            castle_w_l: true,
            castle_b_s: true,
            castle_b_l: true,
        }
    }
    pub fn attack_check_mask(&self, player: bool) -> (BitBoard, BitBoard) {
        let opp_king_mask = self.piece_bb(PieceType::King, !player);

        let mut attack_mask = 0;
        let mut check_mask = 0;

        for (i, bb) in self.bitboards.iter()
                                    .enumerate()
                                    .filter(|(i, _)| get_piece(*i).player == player) 
        {
            let (a_mask, c_mask) = get_piece(i).attack_check_mask(bb, &self.empty_mask(), &opp_king_mask);
            attack_mask |= a_mask;
            check_mask |= c_mask;
        }

        (attack_mask, check_mask)
    }
    pub fn move_list(&self, player: bool, prev_move: &Move, move_list: &mut MoveList) {
        let (opp_attack_mask, opp_check_mask) = self.attack_check_mask(!player);
        let mut pinned_pieces: FnvHashMap<u8, BitBoard> = FnvHashMap::default();
        let pinned_mask = self.pinned(player, &mut pinned_pieces);
        let board_state = BoardState{
            board: self,
            prev_move,
            pinned_mask: &pinned_mask,
            pinned_pieces: &pinned_pieces,
            opp_check_mask: &opp_check_mask,
            opp_attack_mask: &opp_attack_mask,
        };
        self.bitboards.iter()
                .enumerate()
                .filter(|(i, _)| get_piece(*i).player == player)
                .for_each(|(i, bb)| ( get_piece(i).move_list(bb, &board_state, move_list) ) );
    }
    pub fn piece_mask(&self, piece: &Piece) -> BitBoard {
        *self.bitboards.get(get_piece_i(piece)).unwrap_or(&0)
    }
    pub fn empty_mask(&self) -> BitBoard {
        !self.bitboards.iter().fold(0, |acc, v| (acc | v) )
    }
    pub fn piece_bb(&self, piece_type: PieceType, player: bool) -> BitBoard {
        self.bitboards[match_piece_i(piece_type, player)]
    }
    pub fn player_mask(&self, player: bool) -> BitBoard {
        self.bitboards.iter()
                .enumerate()
                .filter(|(i, _)| get_piece(*i).player == player)
                .fold(0, |acc, (_, bb)| (acc | bb))
    }
    pub fn player_bbs(&self, player: bool) -> &[BitBoard] {
        let i = player_pieces_i(player);
        &self.bitboards[i..i+6]
    }
    /// Checks if a square *pos (assumed to be occupied by *player) is attacked. 
    /// Not to be used for pawn squares as it doesn't consider enpassant
    pub fn sq_attacked(&self, pos: u8, player: bool) -> bool {
        let full_occp = !self.empty_mask();
        if let [p, n, b, r, q, k] = self.player_bbs(!player) {
            let a_mask = ( utils::pawn_attack(pos, player) & p) |
                            ( utils::knight_attack(pos) & n ) |
                            ( utils::bishop_attack(pos, full_occp) & (b | q) ) |
                            ( utils::rook_attack(pos, full_occp) & (r | q) ) |
                            ( utils::king_attack(pos) & k );
            a_mask > 0
        } else {
            false
        }
    }
    /// Checks corner case of king in check along 4th rank after enpassant capture
    pub fn is_post_enp_checked(&self, player: bool) -> bool {
        let full_occp = !self.empty_mask();
        let mut checked = false;
        for king_pos in BitPositions( self.piece_bb(PieceType::King, player) ) {
            if let [_, _, _, r, q, _] = self.player_bbs(!player) {
                let a_mask = utils::rook_attack(king_pos, full_occp) & (r | q);
                checked |= a_mask > 0;
            }
        }
        checked
    }
    pub fn castle_rights(&self, player: bool, is_short: bool) -> bool {
        match (player, is_short) {
            (WHITE, true) => self.castle_w_s,
            (WHITE, false) => self.castle_w_l,
            (BLACK, true) => self.castle_b_s,
            (BLACK, false) => self.castle_b_l,
        }
    }
    pub fn unset_castle_rights(&mut self, player: bool, is_short: bool) {
        match (player, is_short) {
            (WHITE, true) => self.castle_w_s = false,
            (WHITE, false) => self.castle_w_l = false,
            (BLACK, true) => self.castle_b_s = false,
            (BLACK, false) => self.castle_b_l = false,
        }
    }
    pub fn from_self(&self, bitboards: [BitBoard; STD_PIECECOUNT]) -> Board {
        Board {
            bitboards: bitboards,
            previous: Some(self),
            castle_w_s: self.castle_w_s,
            castle_w_l: self.castle_w_l,
            castle_b_s: self.castle_b_s,
            castle_b_l: self.castle_b_l,
        }
    }
    pub fn make_move(&self, mov: Move) -> Option<Board> {
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
                bitboards[match_piece_i(PieceType::Pawn, !player)] ^= utils::pos_mask(
                    if player { mov.dest()-8 } else { mov.dest()+8 }
                );
                bitboards[get_piece_i(&piece)] ^= src_mask | dest_mask;
                if self.is_post_enp_checked(player) {
                    return None
                }
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

        let mut next_board = self.from_self(bitboards);

        match mov.piece() {
            Piece{ piece_type: PieceType::King, player } => {
                next_board.unset_castle_rights(player, true);
                next_board.unset_castle_rights(player, false);
            },
            Piece{ piece_type: PieceType::Rook, player } => {
                let (file, _) = utils::file_rank(mov.src());
                let is_short = file == 0;
                next_board.unset_castle_rights(player, is_short);
            },
            _ => {}
        }

        Some( next_board )
    }

    // Get a mapping/list of tuples with pinned_sq -> ray of pin. Also return the combined pinned pieces mask
    pub fn pinned(&self, player: bool, pinned: &mut FnvHashMap<u8, BitBoard>) -> BitBoard {
        let king_bb = self.piece_bb(PieceType::King, player);
        let player_mask = self.player_mask(player);
        let opp_mask = self.player_mask(!player);

        let mut pinned_mask = 0;
        if let [_, _, b, r, q, _] = self.player_bbs(!player) {
            for king_pos in BitPositions(king_bb) {
                let snipers = ( utils::bishop_attack(king_pos, opp_mask) & (b | q) ) |
                            ( utils::rook_attack(king_pos, opp_mask) & (r | q) );
                for sniper_pos in  BitPositions(snipers) {
                    let ray = utils::ray(king_pos, sniper_pos);
                    let blockers = ray & player_mask;
                    let num_blockers = utils::n_set_bits(blockers);
                    if num_blockers == 1 {
                        pinned_mask |= blockers;
                        for pinned_pos in BitPositions(blockers) {
                            pinned.insert(pinned_pos, ray);
                        }
                    }
                }
            }
        }
        pinned_mask
    }
}