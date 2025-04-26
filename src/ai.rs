/*!
 * AI module for Tic Tac Toe
 * This module contains the AI logic for the Tic Tac Toe game.
 * The AI uses a simple strategy to determine the next move.
 * It first checks if there is a winning move available, then checks if it needs to block the opponent's winning move,
 * and finally picks a random available move.
 */

use rand::seq::IteratorRandom;

use crate::game::{Cell, Game};


/// The `get_next_move` function determines the next move for the AI player.
/// It first checks if there is a winning move available.
/// If not, it checks if it needs to block the opponent's winning move.
/// If neither is available, it picks a random available move.
/// The function returns an `Option<(usize, usize)>` representing the coordinates of the next move.
/// If no valid move is available, it returns `None`.
pub fn get_next_move(game: &Game) -> Option<(usize, usize)> {

    // Placeholder for the random number generator
    let mut rng = rand::rng();

    // Check if any move leads to a win
    for y in 0..3 {
        for x in 0..3 {
            if game.get_board_cell(x, y) == Cell::Empty {
                let mut simulated_game = game.clone();
                simulated_game.make_move(x, y);
                if simulated_game.is_over() {
                    return Some((x, y));
                }
            }
        }
    }

    // Check if any move prevents the opponent from winning
    for y in 0..3 {
        for x in 0..3 {
            if game.get_board_cell(x, y) == Cell::Empty {
                let mut simulated_game = game.clone();
                simulated_game.switch_player();
                simulated_game.make_move(x, y);
                if simulated_game.is_over() {
                    return Some((x, y));
                }
            }
        }
    }

    // Otherwise, pick a random move
    (0..3)
        .flat_map(|y| (0..3).map(move |x| (x, y)))
        .filter(|&(x, y)| game.get_board_cell(x, y) == Cell::Empty)
        .choose(&mut rng)
}
