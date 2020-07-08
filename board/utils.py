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
        bb = uint64(bitboard >> 9*steps)
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

# NB: counts from 1..
def get_bit_positions(x: int):
    poss = []
    while(x):
        ls1b = x & -x
        poss.append(ls1b.bit_length())
        x &= x-1
    return poss

def bit_count(pbb):
    count = 0
    while(pbb):
        pbb &= pbb-1
        count += 1
    return count

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
def file_to_rank_bits(n):
    return reduce(int.__or__, [1<<8*b for b in range(1,7) if (n & 1<<b)], 0)

# %%
MAGIC_R_SHIFT = [
	52, 53, 53, 53, 53, 53, 53, 52,
	53, 54, 54, 54, 54, 54, 54, 53,
	53, 54, 54, 54, 54, 54, 54, 53,
	53, 54, 54, 54, 54, 54, 54, 53,
	53, 54, 54, 54, 54, 54, 54, 53,
	53, 54, 54, 54, 54, 54, 54, 53,
	53, 54, 54, 54, 54, 54, 54, 53,
	53, 54, 54, 53, 53, 53, 53, 53
]

MAGIC_R_MAGICN = [
	0x0080001020400080, 0x0040001000200040, 0x0080081000200080, 0x0080040800100080,
	0x0080020400080080, 0x0080010200040080, 0x0080008001000200, 0x0080002040800100,
	0x0000800020400080, 0x0000400020005000, 0x0000801000200080, 0x0000800800100080,
	0x0000800400080080, 0x0000800200040080, 0x0000800100020080, 0x0000800040800100,
	0x0000208000400080, 0x0000404000201000, 0x0000808010002000, 0x0000808008001000,
	0x0000808004000800, 0x0000808002000400, 0x0000010100020004, 0x0000020000408104,
	0x0000208080004000, 0x0000200040005000, 0x0000100080200080, 0x0000080080100080,
	0x0000040080080080, 0x0000020080040080, 0x0000010080800200, 0x0000800080004100,
	0x0000204000800080, 0x0000200040401000, 0x0000100080802000, 0x0000080080801000,
	0x0000040080800800, 0x0000020080800400, 0x0000020001010004, 0x0000800040800100,
	0x0000204000808000, 0x0000200040008080, 0x0000100020008080, 0x0000080010008080,
	0x0000040008008080, 0x0000020004008080, 0x0000010002008080, 0x0000004081020004,
	0x0000204000800080, 0x0000200040008080, 0x0000100020008080, 0x0000080010008080,
	0x0000040008008080, 0x0000020004008080, 0x0000800100020080, 0x0000800041000080,
	0x00FFFCDDFCED714A, 0x007FFCDDFCED714A, 0x003FFFCDFFD88096, 0x0000040810002101,
	0x0001000204080011, 0x0001000204000801, 0x0001000082000401, 0x0001FFFAABFAD1A2
]

MAGIC_R_MASK = [
	0x000101010101017E, 0x000202020202027C, 0x000404040404047A, 0x0008080808080876,
	0x001010101010106E, 0x002020202020205E, 0x004040404040403E, 0x008080808080807E,
	0x0001010101017E00, 0x0002020202027C00, 0x0004040404047A00, 0x0008080808087600,
	0x0010101010106E00, 0x0020202020205E00, 0x0040404040403E00, 0x0080808080807E00,
	0x00010101017E0100, 0x00020202027C0200, 0x00040404047A0400, 0x0008080808760800,
	0x00101010106E1000, 0x00202020205E2000, 0x00404040403E4000, 0x00808080807E8000,
	0x000101017E010100, 0x000202027C020200, 0x000404047A040400, 0x0008080876080800,
	0x001010106E101000, 0x002020205E202000, 0x004040403E404000, 0x008080807E808000,
	0x0001017E01010100, 0x0002027C02020200, 0x0004047A04040400, 0x0008087608080800,
	0x0010106E10101000, 0x0020205E20202000, 0x0040403E40404000, 0x0080807E80808000,
	0x00017E0101010100, 0x00027C0202020200, 0x00047A0404040400, 0x0008760808080800,
	0x00106E1010101000, 0x00205E2020202000, 0x00403E4040404000, 0x00807E8080808000,
	0x007E010101010100, 0x007C020202020200, 0x007A040404040400, 0x0076080808080800,
	0x006E101010101000, 0x005E202020202000, 0x003E404040404000, 0x007E808080808000,
	0x7E01010101010100, 0x7C02020202020200, 0x7A04040404040400, 0x7608080808080800,
	0x6E10101010101000, 0x5E20202020202000, 0x3E40404040404000, 0x7E80808080808000
]

# // And SmallChess too!

MAGIC_B_SHIFT = [
	58, 59, 59, 59, 59, 59, 59, 58,
	59, 59, 59, 59, 59, 59, 59, 59,
	59, 59, 57, 57, 57, 57, 59, 59,
	59, 59, 57, 55, 55, 57, 59, 59,
	59, 59, 57, 55, 55, 57, 59, 59,
	59, 59, 57, 57, 57, 57, 59, 59,
	59, 59, 59, 59, 59, 59, 59, 59,
	58, 59, 59, 59, 59, 59, 59, 58
]

MAGIC_B_MAGICN = [
	0x0002020202020200, 0x0002020202020000, 0x0004010202000000, 0x0004040080000000,
	0x0001104000000000, 0x0000821040000000, 0x0000410410400000, 0x0000104104104000,
	0x0000040404040400, 0x0000020202020200, 0x0000040102020000, 0x0000040400800000,
	0x0000011040000000, 0x0000008210400000, 0x0000004104104000, 0x0000002082082000,
	0x0004000808080800, 0x0002000404040400, 0x0001000202020200, 0x0000800802004000,
	0x0000800400A00000, 0x0000200100884000, 0x0000400082082000, 0x0000200041041000,
	0x0002080010101000, 0x0001040008080800, 0x0000208004010400, 0x0000404004010200,
	0x0000840000802000, 0x0000404002011000, 0x0000808001041000, 0x0000404000820800,
	0x0001041000202000, 0x0000820800101000, 0x0000104400080800, 0x0000020080080080,
	0x0000404040040100, 0x0000808100020100, 0x0001010100020800, 0x0000808080010400,
	0x0000820820004000, 0x0000410410002000, 0x0000082088001000, 0x0000002011000800,
	0x0000080100400400, 0x0001010101000200, 0x0002020202000400, 0x0001010101000200,
	0x0000410410400000, 0x0000208208200000, 0x0000002084100000, 0x0000000020880000,
	0x0000001002020000, 0x0000040408020000, 0x0004040404040000, 0x0002020202020000,
	0x0000104104104000, 0x0000002082082000, 0x0000000020841000, 0x0000000000208800,
	0x0000000010020200, 0x0000000404080200, 0x0000040404040400, 0x0002020202020200
]

MAGIC_B_MASK = [
	0x0040201008040200, 0x0000402010080400, 0x0000004020100A00, 0x0000000040221400,
	0x0000000002442800, 0x0000000204085000, 0x0000020408102000, 0x0002040810204000,
	0x0020100804020000, 0x0040201008040000, 0x00004020100A0000, 0x0000004022140000,
	0x0000000244280000, 0x0000020408500000, 0x0002040810200000, 0x0004081020400000,
	0x0010080402000200, 0x0020100804000400, 0x004020100A000A00, 0x0000402214001400,
	0x0000024428002800, 0x0002040850005000, 0x0004081020002000, 0x0008102040004000,
	0x0008040200020400, 0x0010080400040800, 0x0020100A000A1000, 0x0040221400142200,
	0x0002442800284400, 0x0004085000500800, 0x0008102000201000, 0x0010204000402000,
	0x0004020002040800, 0x0008040004081000, 0x00100A000A102000, 0x0022140014224000,
	0x0044280028440200, 0x0008500050080400, 0x0010200020100800, 0x0020400040201000,
	0x0002000204081000, 0x0004000408102000, 0x000A000A10204000, 0x0014001422400000,
	0x0028002844020000, 0x0050005008040200, 0x0020002010080400, 0x0040004020100800,
	0x0000020408102000, 0x0000040810204000, 0x00000A1020400000, 0x0000142240000000,
	0x0000284402000000, 0x0000500804020000, 0x0000201008040200, 0x0000402010080400,
	0x0002040810204000, 0x0004081020400000, 0x000A102040000000, 0x0014224000000000,
	0x0028440200000000, 0x0050080402000000, 0x0020100804020000, 0x0040201008040200
]

MAGIC_B_OFFSETS = [
    4992, 2624,  256,  896, 1280, 1664, 4800, 5120,
	2560, 2656,  288,  928, 1312, 1696, 4832, 4928,
	   0,  128,  320,  960, 1344, 1728, 2304, 2432,
	  32,  160,  448, 2752, 3776, 1856, 2336, 2464,
	  64,  192,  576, 3264, 4288, 1984, 2368, 2496,
	  96,  224,  704, 1088, 1472, 2112, 2400, 2528,
	2592, 2688,  832, 1216, 1600, 2240, 4864, 4960,
	5056, 2720,  864, 1248, 1632, 2272, 4896, 5184
]

MAGIC_R_OFFSETS = [
    86016, 73728, 36864, 43008, 47104, 51200, 77824, 94208,
	69632, 32768, 38912, 10240, 14336, 53248, 57344, 81920,
	24576, 33792,  6144, 11264, 15360, 18432, 58368, 61440,
	26624,  4096,  7168,     0,  2048, 19456, 22528, 63488,
	28672,  5120,  8192,  1024,  3072, 20480, 23552, 65536,
	30720, 34816,  9216, 12288, 16384, 21504, 59392, 67584,
	71680, 35840, 39936, 13312, 17408, 54272, 60416, 83968,
	90112, 75776, 40960, 45056, 49152, 55296, 79872, 98304
]

#%%

MAGIC_B = [0]*5248;
MAGIC_R = [0]*102400;

#%%
def init_magicmoves():
    rook_occp_count = 0
    rook_ow_count = 0
    for pos in range(64):
        b_mask = MAGIC_B_MASK[pos]
        b_set_bits = get_bit_positions(b_mask)
        # print(f"bishop sq: {pos}, setbits: {len(b_set_bits)}")
        for i_occp in range(1 << len(b_set_bits)):
            occp = reduce(
                int.__or__, 
                [ 1 << (b-1) if(i_occp & (1<<i_b)) else 0 for i_b, b in enumerate(b_set_bits)]
                )
            attack = ray_moves(pos, occp, ['ne', 'se', 'sw', 'nw'])
            if attack == 0:
                print('what!')
                print(f"rook sq: {pos}, setbits: {len(b_set_bits)}")
                print(f"occp: {hex(occp)}, attack: {hex(attack)}, write_at:{MAGIC_B_OFFSETS[pos]}+{ind}")

            ind = uint64(occp*MAGIC_B_MAGICN[pos]) >> MAGIC_B_SHIFT[pos]
            # print(f"occp: {hex(occp)}, attack: {hex(attack)}, write_at:{MAGIC_B_OFFSETS[pos]}+{ind}")
            ind += MAGIC_B_OFFSETS[pos]
            MAGIC_B[ind] = uint64(attack)
        
        r_mask = MAGIC_R_MASK[pos]
        r_set_bits = get_bit_positions(r_mask)
        # print(f"rook sq: {pos}, setbits: {len(b_set_bits)}")
        rook_occp_count += (1 << len(r_set_bits))
        for i_occp in range(1 << len(r_set_bits)):
            occp = reduce(
                int.__or__, 
                [ 1 << (b-1) if(i_occp & (1<<i_b)) else 0 for i_b, b in enumerate(r_set_bits)]
                )
            attack = ray_moves(pos, occp, ['n', 'e', 's', 'w'])
            if attack == 0:
                print('what!')
                print(f"rook sq: {pos}, setbits: {len(b_set_bits)}")
                print(f"occp: {hex(occp)}, attack: {hex(attack)}, write_at:{MAGIC_B_OFFSETS[pos]}+{ind}")
            
            if attack > ones_64:
                print('what >!')
                print(f"rook sq: {pos}, setbits: {len(b_set_bits)}")
                print(f"occp: {hex(occp)}, attack: {hex(attack)}, write_at:{MAGIC_B_OFFSETS[pos]}+{ind}")

            ind = uint64(occp*MAGIC_R_MAGICN[pos]) >> MAGIC_R_SHIFT[pos]
            # print(f"occp: {hex(occp)}, attack: {hex(attack)}, write_at:{MAGIC_B_OFFSETS[pos]}+{ind}")
            ind += MAGIC_R_OFFSETS[pos]
            if MAGIC_R[ind] > 0:
                rook_ow_count += 1
                # print("what! overwrite")
                # print(f"existing: {hex(MAGIC_R[ind])}, new: {hex(attack)}, occp: {hex(occp)}")
            MAGIC_R[ind] = uint64(attack)
    print(f'rook total: {rook_occp_count}, ows: {rook_ow_count}')

#%%
# init_magicmoves()

#%%
def write_magicmoves():
    with open("bishop_magic.txt", "w") as file:
        lw = 16
        file.write(
            "["+
            "\n".join(
                (",".join( 
                    map(hex, MAGIC_B[i*16: i*16 + 16]) 
                    ) + ","
                    for i in range(len(MAGIC_B)//16) )
            ) +
            "]"
        )
    with open("rook_magic.txt", "w") as file:
        lw = 16
        file.write(
            "["+
            "\n".join(
                (",".join( 
                    map(hex, MAGIC_R[i*16: i*16 + 16]) 
                    ) + "," 
                    for i in range(len(MAGIC_R)//16) )
            ) +
            "]"
        )

#%%
# write_magicmoves()

# %%
def get_rays():
    rays = []
    for i in range(64):
        i_file, i_rank = i%8, i//8
        # print(f"from {file_rank_str(i)}")
        from_i = []
        for j in range(64):
            j_file, j_rank = j%8, j//8
            ray = 0
            if i != j:
                if i_file == j_file:
                    dirs=['n', 's']
                elif i_rank == j_rank:
                    dirs=['e', 'w']
                elif i_rank-i_file == j_rank-j_file:
                    dirs=['sw', 'ne']
                elif i_rank+i_file == j_rank+j_file:
                    dirs=['se', 'nw']
                ray = ray_moves(i, occp=(1<<j), dirs=dirs)
                ray &= ray_moves(j, occp=(1<<i), dirs=dirs)
                # if ray != 0:
                # the dest is always in the ray, even when no direct ray exists! useful elsewhere 
                ray |= (1<<j) 
            # print(f"to {file_rank_str(j)}: {ray}")
            # print(i,j,rays[i][j])
            from_i.append(ray)
        rays.append(from_i)
    return rays


# %%
def write_rays():
    a = get_rays()
    with open('./rays.txt', "w") as f:
        f.write("[\n" + ",\n".join(
            "["+",".join(hex(i) for i in line)+"]" for line in a
        ) + "\n]"
)

# %%
