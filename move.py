from typing import Iterable
from collections.abc import Iterable
from itertools import chain

from utils import file_rank_str

class Move:
    def __init__(self, piece, src: int, dest: int):
        super().__init__()
        # TODO: if is capture
        self.piece = piece
        self.src = src
        self.dest = dest

    def __str__(self):
        return f'Piece: {self.piece}, {file_rank_str(self.src-1)} -> {file_rank_str(self.dest-1)}'

class PieceMoveList(Iterable):
    def __init__(self, piece, srcs, dests):
        super().__init__()
        self.piece = piece
        self.srcs = iter(srcs)
        self.dests = iter(dests)

    def __iter__(self):
        sentinel = object()
        while [self.srcs, self.dests]:
            src = next(self.srcs, sentinel)
            dest = next(self.dests, sentinel)
            if sentinel in [src, dest]:
                break
            yield Move(self.piece, src, dest)

class MoveList(Iterable):
    def __init__(self, piece_move_lists):
        super().__init__()
        self.moves = chain.from_iterable(piece_move_lists)
    
    def __iter__(self):
        while self.moves:
            yield next(self.moves)