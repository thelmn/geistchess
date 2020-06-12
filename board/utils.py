# %%
import textwrap
from functools import reduce

# %%
ones_64 = 0xff_ff_ff_ff_ff_ff_ff_ff

class uint64(int):
    def __new__(cls, value):
        return int.__new__(int, value & ones_64)


# %%
files = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h']

def file_rank(pos):
    return files[pos % 8], (pos//8)+1


def file_rank_int(pos):
    return (pos % 8)+1, (pos//8)+1


def file_rank_str(pos):
    f, r = file_rank(pos)
    return f'{f}{r}'

# class Move():
#     def __init__(self, piece, currPos, nextPos, isCapture=False):
#         super().__init__()
#         self.piece = piece
#         self.currPos = currPos
#         self.nextPos = nextPos
#         self.isCapture = isCapture

#     def __repr__(self):
#         piece.move_rep(self)


def file_slide(bitboard, steps=1, forward=True):
    if forward:
        return uint64(bitboard << 8*steps)
    else:
        return uint64(bitboard >> 8*steps)


file_a = 0x01_01_01_01_01_01_01_01
file_h = 0x80_80_80_80_80_80_80_80

# %%


def shift(x, n):
    return x << abs(n) if (n >= 0) else x >> abs(n)


def lshift_mask(x, steps, mask):
    return mask & (x << steps)


def rshift_mask(x, steps, mask):
    return mask & (x >> steps)


def shift_or(x, poss: list):
    return uint64(reduce(int.__or__, [shift(x, i) for i in poss]))

# %%


def byte_repeat(x, n):
    reps = ((x << i*8) for i in range(n))
    return reduce(int.__or__, reps)

# %%


def clear_files(bitboard, files, start=True):
    if start:
        mask = lshift_mask(0xff, files, 0xff)
    else:
        mask = rshift_mask(0xff, files, 0xff)
    mask = byte_repeat(mask, 8)
    return bitboard & mask

# %%


def swne_slide(bitboard, steps=1, forward=True):
    if forward:
        bb = uint64(bitboard << 9*steps)
        return clear_files(bb, steps)
    else:
        bb = uint64(bitboard << 9*steps)
        return clear_files(bb, steps, start=False)


def senw_slide(bitboard, steps=1, forward=True):
    if forward:
        bb = uint64(bitboard << 7*steps)
        return clear_files(bb, steps, start=False)
    else:
        bb = uint64(bitboard >> 7*steps)
        return clear_files(bb, steps)

# %%
def position_bb(pos):
    return 1 << pos if pos >= 0 else 0

def get_bit_positions(x):
    poss = []
    while(x):
        ls1b = x & -x
        poss.append(ls1b.bit_length())
        x &= x-1
    return poss

# %%


def empty_bb(bbs):
    return uint64(~reduce(int.__or__, bbs.values()))


def non_empty_bb(bbs):
    return uint64(reduce(int.__or__, bbs.values()))


def empty_gen():
    yield from ()

# %%
def print_bb(pbb):
    bit_str = bin((1 << 64) | pbb)[3:]
    bb_strs = textwrap.wrap(bit_str, 8)
    bb_strs = [s[::-1] for s in bb_strs]
    bb_str = '\n'.join(bb_strs)
    print(bb_str)

def print_board(board):
    bb_str = bytearray('-'*64, 'ascii')
    for piece, pbb in board.items():
        for pos in get_bit_positions(pbb):
            char = str.encode(piece.abbr if piece.abbr != '' else 'p')
            if piece._player:
                char = char.upper()
            bb_str[64-pos] = ord(char)
    bb_strs = [bb_str[8*i: 8*i+8] for i in range(8)]
    bb_strs = [s[::-1].decode() for s in bb_strs]
    bb_str = '\n'.join(bb_strs)
    print(bb_str)
# %%

# rays: dir -> (steps, step size)
rays = {
    'n': (lambda file, rank: 8-rank, 8),
    'ne': (lambda file, rank: min(8-file, 8-rank), 9),
    'e': (lambda file, rank: 8-file, 1),
    'se': (lambda file, rank: min(8-file, rank-1), -7),
    's': (lambda file, rank: rank-1, -8),
    'sw': (lambda file, rank: min(file-1, rank-1), -9),
    'w': (lambda file, rank: file-1, -1),
    'nw': (lambda file, rank: min(file-1, 8-rank), 7),
}

def ray(src, dir):
    src_bb = uint64(1 << src)
    file, rank = file_rank_int(src)
    steps, step_size = rays[dir]
    steps = steps(file, rank)
    ray_bb = reduce(int.__or__, [shift(src_bb, step_size*s)
                                 for s in range(1, steps+1)], 0)
    return ray_bb, step_size > 0

#%%
def lsb(pbb):
    return (pbb & -pbb).bit_length()

def msb(pbb):
    return pbb.bit_length()

def lsb_bb(pbb):
    return pbb & -pbb

def msb_bb(pbb):
    for i in range(6):
        pbb |= pbb >> 2**i
    pbb += 1
    return pbb >> 1

# %%
def ray_moves(src, occp, dirs):
    r_attack = 0
    for dir in dirs:
        r, is_p = ray(src, dir)
        r_occ = r & occp
        blocker = lsb(r_occ) if is_p else msb(r_occ)
        r_blocker, is_p = ray(blocker-1, dir) if blocker > 0 else (0, True)
        r_attack |= r ^ r_blocker
    return r_attack
#%%
