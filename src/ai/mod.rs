use crate::pieces::{board::Board, traits::Piece};
use std::cmp::{max, min};

/// AI engine
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
    pub fn alpha_beta(&self, board: Board, depth: usize, alpha: i32, beta: i32) -> i32 {
        if depth == 0 || board.is_game_over() {
            return board.evaluate();
        }

        let mut value = 0;
        let mut alpha = alpha;
        let mut beta = beta;
        let moves = board.moves();

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

        alpha
    }
}
