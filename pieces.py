import abc
from typing import List
from itertools import chain

from utils import uint64, shift, shift_or, file_rank_int, file_slide, senw_slide, swne_slide, get_bit_positions, clear_files, empty_bb, non_empty_bb, empty_gen
from player import Player
from move import PieceMoveList

rank_4 = 0x00_00_00_00_ff_00_00_00
rank_5 = 0x00_00_00_ff_00_00_00_00

knight_shifts = [6, 15, 17, 10, -6, -15, -17, -10]
#         noNoWe    noNoEa
#             +15  +17
#              |     |
# noWeWe  +6 __|     |__+10  noEaEa
#               \   /
#                >0<
#            __ /   \ __
# soWeWe -10   |     |   -6  soEaEa
#              |     |
#             -17  -15
#         soSoWe    soSoEa

class Piece(metaclass=abc.ABCMeta):
    def __init__(self, player: Player):
        super().__init__()
        self._player = player

    @abc.abstractmethod
    def get_valid_moves(self, board: List[uint64], prevMove):
        pass

    @property
    def player(self):
        return self._player

    @property
    @abc.abstractproperty
    def rep(self):
        pass
    
    @property
    @abc.abstractproperty
    def standard_full(self):
        pass

    def __str__(self):
        return f'{self.__class__.__name__}[{"White" if self.player.forward else "Black"}]'

class Pawn(Piece):
    def __init__(self, player: Player):
        super().__init__(player)

    def get_valid_moves(self, board, prevMove):
        moves = []
        mov_srcs = []
        # get valid moves for all pawns.
        forward = self._player.forward
        mepawns = board.bb4piece(self)
        # pushable pawns
        # single pawn push. push
        empty = empty_bb(board.bitboards)
        pp1 = file_slide(empty, forward=not forward) & mepawns
        # double pawn push
        drank = rank_4 if forward else rank_5
        emptyRank3 = file_slide(empty & drank, forward=not forward) & empty
        pp2 = file_slide(emptyRank3, forward=not forward) & mepawns

        # can capture
        bbs4_oppp = non_empty_bb(board.bbs4_opp_player(self._player))
        # nw
        cp_nw = senw_slide(bbs4_oppp, 1, forward=not forward) & mepawns
        # ne
        cp_ne = swne_slide(bbs4_oppp, 1, forward=not forward) & mepawns

        # TODO: enpassant capture

        pp1_src = get_bit_positions(pp1)
        pp2_src = get_bit_positions(pp2)
        cpnw_src = get_bit_positions(cp_nw)
        cpne_src = get_bit_positions(cp_ne)

        pp1_dest = list(map(lambda x: x+8 if forward else x-8, pp1_src))
        pp2_dest = list(map(lambda x: x+16 if forward else x-16, pp2_src))
        cpnw_dest = list(map(lambda x: x+7 if forward else x-8, cpnw_src))
        cpne_dest = list(map(lambda x: x+9 if forward else x-8, cpne_src))

        return PieceMoveList(
            self,
            chain(pp1_src, pp2_src, cpnw_src, cpne_src),
            chain(pp1_dest, pp2_dest, cpnw_dest, cpne_dest),
            chain([False]*(len(pp1_dest)+len(pp2_dest)), [True]*(len(cpnw_dest)+len(cpne_dest)))
        )

    @property
    def rep(self):
        return None

    @property
    def standard_full(self):
        return uint64(0xff_00) if self._player.forward else uint64(0x00_ff_00_00_00_00_00_00)

class Knight(Piece):
    def __init__(self, player):
        super().__init__(player)
    
    def get_valid_moves(self, board: List[uint64], prevMove):
        forward = self._player.forward
        meknights = board.bb4piece(self)

        empty = empty_bb(board.bitboards)
        bbs4_oppp = non_empty_bb(board.bbs4_opp_player(self._player))

        isolated_kn = get_bit_positions(meknights)
        src_empty = []
        src_capture = []
        dest_empty = []
        dest_capture = []
        for bit_pos in isolated_kn:
            kn_dest_bb = shift_or(shift(1, bit_pos-1), knight_shifts)

            # clear last/first 2 files to avoid wrap around
            file, _ = file_rank_int(bit_pos-1)
            if file in [1,2,7,8]:
                kn_dest_bb = clear_files(kn_dest_bb, 2, start=file in [7,8])

            kn_dest_empty = get_bit_positions(kn_dest_bb & empty)
            kn_dest_capture = get_bit_positions(kn_dest_bb & bbs4_oppp)

            src_empty.extend([bit_pos]*len(kn_dest_empty))
            src_capture.extend([bit_pos]*len(kn_dest_capture))

            dest_empty.extend(kn_dest_empty)
            dest_capture.extend(kn_dest_capture)
        
        return PieceMoveList(
            self,
            chain(src_empty, src_capture),
            chain(dest_empty, dest_capture),
            chain([False]*len(dest_empty), [True]*len(dest_capture))
        )


    @property
    def rep(self):
        return None

    @property
    def standard_full(self):
        return uint64(0x42) if self._player.forward else uint64(0x42_00_00_00_00_00_00_00)


class Bishop(Piece):
    def __init__(self, player):
        super().__init__(player)
    
    def get_valid_moves(self, selfBoard: List[uint64], prevMove):
        return empty_gen()

    @property
    def rep(self):
        return None

    @property
    def standard_full(self):
        return uint64(0x24) if self._player.forward else uint64(0x24_00_00_00_00_00_00_00)


class Rook(Piece):
    def __init__(self, player):
        super().__init__(player)
    
    def get_valid_moves(self, selfBoard: List[uint64], prevMove):
        return empty_gen()

    @property
    def rep(self):
        return None

    @property
    def standard_full(self):
        return uint64(0x81) if self._player.forward else uint64(0x81_00_00_00_00_00_00_00)


class Queen(Piece):
    def __init__(self, player):
        super().__init__(player)

    def get_valid_moves(self, selfBoard: List[uint64], prevMove):
        return empty_gen()

    @property
    def rep(self):
        return None

    @property
    def standard_full(self):
        return uint64(0x08) if self._player.forward else uint64(0x08_00_00_00_00_00_00_00)


class King(Piece):
    def __init__(self, player):
        super().__init__(player)

    def get_valid_moves(self, selfBoard: List[uint64], prevMove):
        return empty_gen()

    @property
    def rep(self):
        return None

    @property
    def standard_full(self):
        return uint64(0x10) if self._player.forward else uint64(0x10_00_00_00_00_00_00_00)
