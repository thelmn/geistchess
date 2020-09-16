use hdf5;
use ndarray;

use crate::board::Board;
use crate::pieces::{PieceType, SqOccupation, std_pieces::*};
use crate::moves::{MoveList, BitPositions};
use crate::utils;

pub type AdjMatrix = [u64; 64];

/// Simple struct to hold the graph of a board. 
/// The intension if to save multiple graphs into a dataset with protobuf
pub struct Graph {
    pub moves:              ndarray::Array2::<bool>,
    pub occup_piecetype:    ndarray::Array1::<PieceType>,
    pub occup_player:       ndarray::Array1::<SqOccupation>,
    pub pinned:             ndarray::Array1::<bool>,
    // pub file_rank:          ndarray::Array1::<(u8, u8)>,
}

pub type Graphs = (
    ndarray::Array3::<bool>, 
    ndarray::Array2::<PieceType>, 
    ndarray::Array2::<SqOccupation>, 
    ndarray::Array2::<bool>
);

impl Default for Graph {
    fn default() -> Self {
        Graph{
            moves:              ndarray::Array2::default( (64, 64) ),
            occup_piecetype:    ndarray::Array1::default( (64,) ),
            occup_player:       ndarray::Array1::default( (64,) ),
            pinned:             ndarray::Array1::default( (64,) ),
            // file_rank:          ndarray::Array1::<(u8, u8)>,
        }
    }
}

#[derive(hdf5::H5Type, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[repr(u8)]
pub enum GameResult {
    Draw = 0,
    White = 1,
    Black = 2,
}

pub fn board_2_graph(board: &Board, graph: &mut Graph, move_list: &mut MoveList) {
    // white moves
    let player = true;
    let pinned = board.pseudo_move_list(player, move_list);
    for mov in move_list.iter() {
        graph.moves[ (mov.src() as usize, mov.dest() as usize) ] = true;
    }
    for pinned_pos in BitPositions(pinned) {
        graph.pinned[ (pinned_pos as usize) ] = true;
    }

    // black moves
    move_list.clear();
    let player = false;
    let pinned = board.pseudo_move_list(player, move_list);
    for mov in move_list.iter() {
        graph.moves[ (mov.src() as usize, mov.dest() as usize) ] = true;
    }
    for pinned_pos in BitPositions(pinned) {
        graph.pinned[ (pinned_pos as usize) ] = true;
    }

    // occupation
    for (i, bb) in board.bitboards.iter().enumerate() {
        let piece = get_piece(i);
        for pos in BitPositions(*bb) {
            graph.occup_piecetype[ (pos as usize) ] = piece.piece_type;
            graph.occup_player[ (pos as usize) ] = piece.player.into();
        }
    }
}

pub fn boards_2_graphs<'a>(
    n: usize, 
    boards: impl Iterator::<Item=&'a Board>
) -> Graphs {
    // let n = &boards.count();
    let mut moves           = ndarray::Array3::default( (n, 64, 64) );
    let mut occup_piecetype = ndarray::Array2::default( (n, 64,) );
    let mut occup_player    = ndarray::Array2::default( (n, 64,) );
    let mut pinned          = ndarray::Array2::default( (n, 64,) );
    let mut graph = Graph::default();
    let mut move_list = MoveList::new();
    for (board, i) in boards.zip(0..n) {
        board_2_graph(board, &mut graph, &mut move_list);
        // assign entry
        let mut row = moves.index_axis_mut(ndarray::Axis(0), i);
        row.assign(&graph.moves);
        let mut row = occup_piecetype.index_axis_mut(ndarray::Axis(0), i);
        row.assign(&graph.occup_piecetype);
        let mut row = occup_player.index_axis_mut(ndarray::Axis(0), i);
        row.assign(&graph.occup_player);
        let mut row = pinned.index_axis_mut(ndarray::Axis(0), i);
        row.assign(&graph.pinned);
        // clear graph for reuse
        graph.moves.fill(false);
        graph.occup_piecetype.fill(PieceType::default());
        graph.occup_player.fill(SqOccupation::default());
        graph.pinned.fill(false);
    }
    (moves, occup_piecetype, occup_player, pinned)
}

pub fn save_graphs(
    filename: &str, n: usize, 
    graphs: Graphs,
    next_player: &[bool],
    scores: &[isize],
    results: &[i8],
) -> Result<(), Box<dyn std::error::Error>> {
    let filename = if filename.ends_with(".h5") {
        filename.to_string() 
    } else {
        [filename, ".h5"].concat() 
    };
    let file = hdf5::File::create(filename)?;
    let piece_types = file.new_dataset::<PieceType>().create("piece_types", 7)?;
    piece_types.write(&[
        PieceType::Invalid, 
        PieceType::Pawn, 
        PieceType::Knight, 
        PieceType::Bishop, 
        PieceType::Rook, 
        PieceType::Queen, 
        PieceType::King,
    ])?;
    let sq_occupation = file.new_dataset::<SqOccupation>().create("sq_occupation", 3)?;
    sq_occupation.write(&[
        SqOccupation::Empty,
        SqOccupation::White,
        SqOccupation::Black,
    ])?;
    let file_rank = file.new_dataset::<(u8,u8)>().create("file_rank", (64,))?;
    file_rank.write( &ndarray::aview1(&utils::FILE_RANK) )?;

    let group = file.create_group("graphs")?;
    let moves = group.new_dataset::<bool>().create("moves", (n, 64, 64))?;
    moves.write( &graphs.0 )?;
    let occup_piecetype = group.new_dataset::<PieceType>().create("occup_piecetype", (n, 64))?;
    occup_piecetype.write( &graphs.1 )?;
    let occup_player = group.new_dataset::<SqOccupation>().create("occup_player", (n, 64))?;
    occup_player.write( &graphs.2 )?;
    let pinned = group.new_dataset::<bool>().create("pinned", (n, 64))?;
    pinned.write( &graphs.3 )?;
    let next_player_ds = group.new_dataset::<bool>().create("next_player", n)?;
    next_player_ds.write( next_player )?;
    let scores_ds = group.new_dataset::<isize>().create("scores", n)?;
    scores_ds.write( scores )?;
    let results_ds = group.new_dataset::<i8>().create("results", n)?;
    results_ds.write( results )?;
    Ok( () )
}

impl From<&Board> for Graph {
    fn from(board: &Board) -> Graph {
        let mut graph = Graph::default();
        let mut move_list = MoveList::new();
        board_2_graph(board, &mut graph, &mut move_list);
        graph
    }
}
