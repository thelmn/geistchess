#%%
from functools import reduce
from .pieces import Pawn, Knight, Bishop, Rook, Queen, King
from .move import MoveList, Move

from .utils import uint64, bit_count, position_bb, print_board, print_bb, non_empty_bb

from typing import List

WHITE = True
BLACK = False

#%%
class Board():
    def __init__(self, pieceList: list, bitboards: List[uint64] = None):
        super().__init__()
        if bitboards is None:
            self._bitboards = dict(zip(pieceList, [uint64(0)]*len(pieceList)))
        else:
            self._bitboards = dict(zip(pieceList, bitboards))
        self.__history_bbs = []
    
    @property
    def bitboards(self):
        return self._bitboards
    
    def get_valid_moves(self, player = None, prev_move = None):
        bbs = self._bitboards if (player is None) else self.bbs4_player(player)
        moves = (piece.get_valid_moves(self, prev_move) for piece, bb in bbs.items())
        return MoveList(moves)

    def make_move(self, move: Move):
        src_bb = position_bb(move.src-1)
        dest_bb = position_bb(move.dest-1)
        self.__history_bbs.append(self._bitboards)
        # unset dest in any piece bb that is set
        self._bitboards = {p: uint64(bb & ~dest_bb) for p, bb in self._bitboards.items()}
        # unset src and set dest in moved piece bb
        # print(f'piece: {move.piece}, src: {move.src}, dest: {move.dest}')
        # print_bb(self._bitboards[move.piece])
        # print('---')
        self._bitboards[move.piece] = self._bitboards[move.piece] ^ src_bb ^ dest_bb
        # print_bb(self._bitboards[move.piece])
        # print(';;')
        # print_bb(uint64(non_empty_bb(self._bitboards)))

    def unmake_move(self):
        self._bitboards = self.__history_bbs.pop()

    @classmethod
    def standardFull(cls):
        std_piece_list = [
            Pawn(WHITE),
            Knight(WHITE),
            Bishop(WHITE),
            Rook(WHITE),
            Queen(WHITE),
            King(WHITE),

            Pawn(BLACK),
            Knight(BLACK),
            Bishop(BLACK),
            Rook(BLACK),
            Queen(BLACK),
            King(BLACK),
        ]
        std_bb = [ p.standard_full for p in std_piece_list ]
        return Board(std_piece_list, std_bb)

    def bb4piece(self, piece):
        return self._bitboards[piece]
    
    def bbs4_player(self, player):
        return dict(filter(lambda pbb: pbb[0].player == player, self._bitboards.items()))
    
    def bbs4_opp_player(self, player):
        return dict(filter(lambda pbb: pbb[0].player != player, self._bitboards.items()))

    def piece_count(self):
        return {p:bit_count(pbb) for p, pbb in self._bitboards.items()}
    
    def fill(self):
        pass

#%%
import random
if __name__ == "__main__":
    b = Board.standardFull()
    print_board(b.bitboards)
    move = None
    for n in range(20):
        move = random.choice(list(b.get_valid_moves((n+1)%2, move)))
        print(move)
        b.make_move(move)

    print_board(b.bitboards)
# %%
