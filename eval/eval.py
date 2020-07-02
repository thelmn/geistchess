import abc
from board.pieces import Pawn, Knight, Bishop, Rook, Queen, King
from board.board import Board

class EvalFN(metaclass=abc.ABCMeta):
    def __init__(self):
        super().__init__()

    @abc.abstractmethod
    def eval(self, board, phase) -> int:
        pass

class material(EvalFN):
    weights = {
        Pawn: 100,
        Knight: 345,
        Bishop: 355,
        Rook: 525,
        Queen: 1000,
        King: 0
    }
    def eval(self, board: Board, phase):
        sign = {True: 1, False: -1}
        score_cp = sum([sign(p.player)*count*self.weights[type(p)] for p, count in board.piece_count().items()])
        return score_cp

class mobility(EvalFN):
    pass