from board import Board
from player import Player


class Game:
    def __init__(self, start_pos = Board.standardFull, next_player = Player(True)):
        super().__init__()
        self._start_pos = start_pos
        self._pos = start_pos
        self._next_player = next_player

    def generate_moves(self):
        pass