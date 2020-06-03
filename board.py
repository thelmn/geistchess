#%%
from functools import reduce
from pieces import Pawn, Knight, Bishop, Rook, Queen, King
from player import Player
from move import MoveList

from utils import uint64

from typing import List

#%%
class Board():
    def __init__(self, pieceList: list, bitboards: List[uint64] = None):
        super().__init__()
        if bitboards is None:
            self._bitboards = dict(zip(pieceList, [uint64(0)]*len(pieceList)))
        else:
            self._bitboards = dict(zip(pieceList, bitboards))
    
    @property
    def bitboards(self):
        return self._bitboards
    
    def get_valid_moves(self, player = None):
        bbs = self._bitboards if (player is None) else self.bbs4_player(player)
        moves = (piece.get_valid_moves(self, prevMove=None) for piece, bb in bbs.items())
        return MoveList(moves)

    @classmethod
    def standardFull(cls):
        white = Player(True)
        black = Player(False)
        stdPieceList = [
            Pawn(white),
            Knight(white),
            Bishop(white),
            Rook(white),
            Queen(white),
            King(white),

            Pawn(black),
            Knight(black),
            Bishop(black),
            Rook(black),
            Queen(black),
            King(black),
        ]
        stdBBs = [ p.standard_full for p in stdPieceList ]
        return Board(stdPieceList, stdBBs)

    def bb4piece(self, piece):
        return self._bitboards[piece]
    
    def bbs4_player(self, player):
        return dict(filter(lambda pbb: pbb[0].player == player, self._bitboards.items()))
    
    def bbs4_opp_player(self, player):
        return dict(filter(lambda pbb: pbb[0].player != player, self._bitboards.items()))
    
    def fill(self):
        pass

#%%
if __name__ == "__main__":
    b = Board.standardFull()
    # print(b.get_valid_moves())
    for m in b.get_valid_moves():
        print(m)

# %%
