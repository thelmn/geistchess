pub mod board;
pub mod moves;
pub mod pieces;
pub mod utils;
pub mod perft;

#[cfg(test)]
mod tests {
    use crate::perft;

    #[test]
    fn perft_4() {
        let move_counts = perft::run(4);
        println!("{:?}", move_counts);
        for n in 0..=3 {
            assert_eq!(move_counts[&n], perft::PERFT_NODE_COUNT[n as usize] as usize)
        }
    }
}