#%%
from functools import reduce

#%%
ones_64 = 0xff_ff_ff_ff_ff_ff_ff_ff

class uint64(int):
    def __new__(cls, value):
        return int.__new__(int, value & ones_64)


files = ['a','b','c','d','e','f','g','h']
def file_rank(pos):
    return files[pos%8], pos//8


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

#%%

def lshift_mask(x, steps, mask):
    return mask & (x << steps)

def rshift_mask(x, steps, mask):
    return mask & (x >> steps)

#%%
def byte_repeat(x, n):
    reps = ((x << i*8) for i in range(n))
    return reduce(int.__or__, reps)

#%%
def clear_files(bitboard, files, start=True):
    if start:
        mask = lshift_mask(0xff, files, 0xff)
    else:
        mask = rshift_mask(0xff, files, 0xff)
    mask = byte_repeat(mask, 8)
    return bitboard & mask

#%%
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

#%%
def get_bit_positions(x):
    poss = []
    while(x):
        ls1b = x & -x
        poss.append(ls1b.bit_length())
        x &= x-1
    return poss

#%%
def empty_bb(bbs):
        return uint64(~reduce(int.__or__, bbs.values()))

def non_empty_bb(bbs):
    return uint64(reduce(int.__or__, bbs.values()))

def empty_gen():
    yield from ()