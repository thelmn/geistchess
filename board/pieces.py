import abc
from typing import List
from itertools import chain
import sys
sys.path.append('../../')


from utils import uint64, shift, shift_or, file_rank_int, file_slide, senw_slide, swne_slide, get_bit_positions, clear_files, empty_bb, non_empty_bb, empty_gen, ray_moves, print_bb
from move import PieceMoveList, Move

#%%
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
king_shifts = [-1, 7, 8, 9, 1, -7, -8, -9]
#          +7  +8  +9
#          -1  >0< +1
#          -9  -8  -7

#%%
class Piece(metaclass=abc.ABCMeta):
    _instance = None

    @staticmethod
    @abc.abstractstaticmethod
    def get_instance():
        pass

    def __init__(self, player: bool):
        super().__init__()
        self._player = player

    @abc.abstractmethod
    def get_valid_moves(self, board, prev_move: Move):
        pass

    @property
    def player(self):
        return self._player

    @property
    @abc.abstractproperty
    def abbr(self):
        pass

    @property
    def key(self):
        return (self.abbr, self.player)

    @property
    @abc.abstractproperty
    def standard_full(self):
        pass

    # def __repr__(self):
    #     return self.__str__()

    def __str__(self):
        return f'{self.__class__.__name__}[{"White" if self._player else "Black"}]'


class Pawn(Piece):
    def __init__(self, player: bool):
        if Pawn._instance is not None and Pawn._instance._player == player:
            raise Exception('class is singleton. use get_instance')
        super().__init__(player)
        Pawn._instance = self

    @staticmethod
    def get_instance(player):
        if Pawn._instance is None or Pawn._instance._player != player:
            return Pawn(player)
        else:
            return Pawn._instance

    def get_valid_moves(self, board, prev_move: Move):
        moves = []
        mov_srcs = []
        # get valid moves for all pawns.
        forward = self.player
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

        # en passant capture
        # if previous move was a double pawn push
        enpc_src = []
        enpc_dest = []
        if prev_move is not None and isinstance(prev_move.piece, Pawn):
            if abs(prev_move.src - prev_move.dest) == 16 and self.player != prev_move.piece._player:
                # get neighbouring squares occupied by opp player pawns
                dest = prev_move.dest+8 if forward else prev_move.dest-8
                neighbours = shift_or(shift(1, prev_move.dest-1), [+1, -1])
                mepawn_nbs = neighbours & mepawns
                enpc_src = get_bit_positions(mepawn_nbs)
                enpc_dest = [dest] * len(enpc_src)

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
            chain(pp1_src, pp2_src, cpnw_src, cpne_src, enpc_src),
            chain(pp1_dest, pp2_dest, cpnw_dest, cpne_dest, enpc_dest),
            chain([False]*(len(pp1_dest)+len(pp2_dest)), [True] *
                  (len(cpnw_dest)+len(cpne_dest)+len(enpc_dest)))
        )

    @property
    def abbr(self):
        return ''

    @property
    def standard_full(self):
        return uint64(0xff_00) if self._player else uint64(0x00_ff_00_00_00_00_00_00)


class Knight(Piece):
    def __init__(self, player: bool):
        if Knight._instance is not None and Knight._instance._player == player:
            raise Exception('class is singleton. use get_instance')
        super().__init__(player)
        Knight._instance = self

    @staticmethod
    def get_instance(player):
        if Knight._instance is None or Knight._instance._player != player:
            return Knight(player)
        else:
            return Knight._instance

    def get_valid_moves(self, board, prev_move):
        forward = self._player
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
            if file in [1, 2, 7, 8]:
                kn_dest_bb = clear_files(kn_dest_bb, 2, start=file in [7, 8])

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
    def abbr(self):
        return 'n'

    @property
    def standard_full(self):
        return uint64(0x42) if self._player else uint64(0x42_00_00_00_00_00_00_00)


def valid_ray_moves(board, piece, dirs):
    pbb = board.bb4piece(piece)
    empty = empty_bb(board.bitboards)
    bbs4_oppp = non_empty_bb(board.bbs4_opp_player(piece._player))
    occp = uint64(~empty)
    src_empty = []
    src_capture = []
    dest_empty = []
    dest_capture = []
    for bit_pos in get_bit_positions(pbb):
        r_attack = ray_moves(bit_pos-1, occp, dirs)
        p_dest_empty = get_bit_positions(r_attack & empty)
        p_dest_capture = get_bit_positions(r_attack & bbs4_oppp)

        src_empty.extend([bit_pos]*len(p_dest_empty))
        src_capture.extend([bit_pos]*len(p_dest_capture))

        dest_empty.extend(p_dest_empty)
        dest_capture.extend(p_dest_capture)
    return PieceMoveList(
        piece,
        chain(src_empty, src_capture),
        chain(dest_empty, dest_capture),
        chain([False]*len(dest_empty), [True]*len(dest_capture))
    )


class Bishop(Piece):
    def __init__(self, player: bool):
        if Bishop._instance is not None and Bishop._instance._player == player:
            raise Exception('class is singleton. use get_instance')
        super().__init__(player)
        Bishop._instance = self
        self.attack_dirs = ['ne', 'se', 'sw', 'nw']
    
    @staticmethod
    def get_instance(player):
        if Bishop._instance is None or Bishop._instance._player != player:
            return Bishop(player)
        else:
            return Bishop._instance

    def get_valid_moves(self, board, prev_move):
        return valid_ray_moves(board, self, self.attack_dirs)

    @property
    def abbr(self):
        return 'b'

    @property
    def standard_full(self):
        return uint64(0x24) if self._player else uint64(0x24_00_00_00_00_00_00_00)


class Rook(Piece):
    def __init__(self, player: bool):
        if Rook._instance is not None and Rook._instance._player == player:
            raise Exception('class is singleton. use get_instance')
        super().__init__(player)
        Rook._instance = self
        self.attack_dirs = ['n', 'e', 's', 'w']

    @staticmethod
    def get_instance(player):
        if Rook._instance is None or Rook._instance._player != player:
            return Rook(player)
        else:
            return Rook._instance

    def get_valid_moves(self, board, prev_move):
        return valid_ray_moves(board, self, self.attack_dirs)

    @property
    def abbr(self):
        return 'r'

    @property
    def standard_full(self):
        return uint64(0x81) if self._player else uint64(0x81_00_00_00_00_00_00_00)


class Queen(Piece):
    def __init__(self, player: bool):
        if Queen._instance is not None and Queen._instance._player == player:
            raise Exception('class is singleton. use get_instance')
        super().__init__(player)
        Queen._instance = self
        self.attack_dirs = ['n', 'ne', 'e', 'se', 's', 'sw', 'w', 'nw']
    
    @staticmethod
    def get_instance(player):
        if Queen._instance is None or Queen._instance._player != player:
            return Queen(player)
        else:
            return Queen._instance

    def get_valid_moves(self, board, prev_move):
        return valid_ray_moves(board, self, self.attack_dirs)

    @property
    def abbr(self):
        return 'q'

    @property
    def standard_full(self):
        return uint64(0x08) if self._player else uint64(0x08_00_00_00_00_00_00_00)


class King(Piece):
    def __init__(self, player: bool):
        if King._instance is not None and King._instance._player == player:
            raise Exception('class is singleton. use get_instance')
        super().__init__(player)
        King._instance = self

    @staticmethod
    def get_instance(player):
        if King._instance is None or King._instance._player != player:
            return King(player)
        else:
            return King._instance

    def get_valid_moves(self, board, prev_move):
        pbb = board.bb4piece(self)
        empty = empty_bb(board.bitboards)
        bbs4_oppp = non_empty_bb(board.bbs4_opp_player(self._player))
        src_empty = []
        src_capture = []
        dest_empty = []
        dest_capture = []
        for bit_pos in get_bit_positions(pbb):
            attack_mask = shift_or(shift(1, bit_pos-1), king_shifts)
            p_dest_empty = get_bit_positions(attack_mask & empty)
            p_dest_capture = get_bit_positions(attack_mask & bbs4_oppp)

            src_empty.extend([bit_pos]*len(p_dest_empty))
            src_capture.extend([bit_pos]*len(p_dest_capture))

            dest_empty.extend(p_dest_empty)
            dest_capture.extend(p_dest_capture)
        return PieceMoveList(
            self,
            chain(src_empty, src_capture),
            chain(dest_empty, dest_capture),
            chain([False]*len(dest_empty), [True]*len(dest_capture))
        )

    @property
    def abbr(self):
        return 'k'

    @property
    def standard_full(self):
        return uint64(0x10) if self._player else uint64(0x10_00_00_00_00_00_00_00)

#%%
ones_64 = 0xff_ff_ff_ff_ff_ff_ff_ff
def get_knight_patterns(meknights):
    isolated_kn = get_bit_positions(meknights)

    mapping = []
    print('knights: ', len(isolated_kn))
    for bit_pos in isolated_kn:
        kn_dest_bb = shift_or(shift(1, bit_pos-1), knight_shifts)

        # clear last/first 2 files to avoid wrap around
        file, _ = file_rank_int(bit_pos-1)
        if file in [1, 2, 7, 8]:
            kn_dest_bb = clear_files(kn_dest_bb, 2, start=file in [7, 8])
        
        # print_bb(kn_dest_bb)
        mapping.append(kn_dest_bb)
    return mapping


# %%

def get_king_patterns(mekings):
    isolated_kn = get_bit_positions(mekings)

    mapping = []
    print('kings: ', len(isolated_kn))
    for bit_pos in isolated_kn:
        kn_dest_bb = shift_or(shift(1, bit_pos-1), king_shifts)

        # clear last/first file to avoid wrap around
        file, _ = file_rank_int(bit_pos-1)
        if file in [1, 8]:
            kn_dest_bb = clear_files(kn_dest_bb, 1, start=file == 8)
        
        # print_bb(kn_dest_bb)
        mapping.append(kn_dest_bb)
    return mapping

# %%
