from board.board import Board
from board.pieces import Pawn, Knight, Bishop, Rook, Queen, King

phase_weights = {
    Pawn: 1,
    Knight: 2,
    Bishop: 2,
    Rook: 4,
    Queen: 6,
    King: 0
}

original_counts = {
    Pawn: 16,
    Knight: 4,
    Bishop: 4,
    Rook: 4,
    Queen: 2,
    King: 2
}

total_phaseweights = sum(phase_weights[p]*count for p, count in original_counts.items())

def gamephase(board: Board, move_count):
    phase = sum(phase_weights[type(p)]*count for p, count in board.piece_count().items())
    phase = 
    pass