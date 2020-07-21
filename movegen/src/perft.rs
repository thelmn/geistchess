
use crate::board::Board;
use crate::moves::MoveList;
use fnv::FnvHashMap;

pub const PERFT_NODE_COUNT: [u32; 7] = [ 
    1,
    20,
    400,
    8_902,
    197_281,
    4_865_609,
    119_060_324,
    // 3_195_901_860, // Let's be reasonable people
    // 84_998_978_956,
    // 2_439_530_234_167,
    // 69_352_859_712_417,
    // 2_097_651_003_696_806,
    // 62_854_969_236_701_747,
    // 1_981_066_775_000_396_239,
    // 61_885_021_521_585_529_237,
    // 2_015_099_950_053_364_471_960,
];

/// Computes perft([0..depth]) starting 
pub fn run(depth: u16) -> FnvHashMap<u16, usize> {
    let board = Board::standard();
    run_for(board, depth)
}

pub fn run_for(start_board: Board, depth: u16) -> FnvHashMap<u16, usize> {
    // Note that using vec as a stack will traverse the tree in depth first
    // with last moves in the movelist explored first
    // If this technique is used in a-b search, the movelist will have to be
    // sorted worst->best for optimal prunage
    let mut queue = Vec::<Board>::with_capacity(PERFT_NODE_COUNT[depth as usize] as usize);
    let mut move_list = MoveList::new();
    let mut node_counts = FnvHashMap::with_capacity_and_hasher((depth+1) as usize, Default::default());

    queue.push(start_board);

    while let Some(board) = queue.pop() {
        // increment node count
        node_counts.entry(board.half_move_count).and_modify(|n| *n += 1).or_insert(1);
        
        // don't insert new nodes if depth reached
        if board.half_move_count == depth {
            continue;
        }

        
        // clear move list for reuse
        move_list.clear();
        board.move_list(board.player, &mut move_list);
        move_list.iter()
                .filter_map(|mov| board.make_move(mov))
                .for_each(|next_board| {
                    queue.push(next_board);
                });

    }
    node_counts
}