use crate::pieces::{board::Board, traits::Piece};
use std::cmp::{max, min};

/// AI engine
/// board_stack: Vec of boards
pub struct Ai {
    pub board_stack: Vec<Board>,
}

impl Default for Ai {
    /// Default AI
    fn default() -> Self {
        let mut board_stack: Vec<Board> = Vec::new();

        Ai { board_stack }
    }
}

impl Ai {
    /// Alpha beta search
    /// board: current board
    /// depth: current depth
    /// alpha: lower bound
    /// beta: upper bound
    pub fn alpha_beta(&self, board: Board, depth: usize, alpha: i32, beta: i32) -> i32 {
        // If the depth is 0 or the game is over, return the evaluation
        if depth == 0 || board.is_game_over() {
            return board.evaluate();
        }

        let mut value = 0;
        let mut alpha = alpha;
        let mut beta = beta;
        let moves = board.moves();

        // Iterate over all the moves and evaluate them
        for mv in moves {
            value = -self.alpha_beta(board, depth - 1, -beta, -alpha);

            // If this is the best move we can make, store it
            // otherwise, prune the search
            if value > alpha {
                if value >= beta {
                    return beta;
                }
                alpha = value;
            }
        }

        // Return the best move
        alpha
    }
}
