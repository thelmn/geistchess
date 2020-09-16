pub mod board;
pub mod moves;
pub mod pieces;
pub mod utils;
pub mod perft;
pub mod graph;
pub mod uci;
pub mod error;
pub mod macros;

#[macro_use] extern crate lazy_static;

#[cfg(test)]
mod tests {
    use crate::perft;
    use crate::uci;

    #[test]
    fn perft_4() {
        let move_counts = perft::run(4);
        println!("{:?}", move_counts);
        for n in 0..=3 {
            assert_eq!(move_counts[&n], perft::PERFT_NODE_COUNT[n as usize] as usize)
        }
    }

    #[test]
    fn test_uci_connect() {
        // assert!(false);
        let eng = uci::UCIClient::try_new("stockfish_11");
        assert!(eng.is_ok());
        if let Ok(mut eng) = eng {
            let uci_ok = eng.init_uci();
            assert!(uci_ok.is_ok());
            // let ready_ok = eng.is_ready();
            // assert!(ready_ok.is_ok());
        }
    }
}