/*!
This module contains the implementation of the Tic Tac Toe game logic.
It includes the game state, player turns, and win conditions.
The game is played on a 3x3 grid, where two players take turns placing their symbols (X or O) in the cells.
The game ends when one player has three symbols in a row, column, or diagonal.
*/

/// Game board winning lines.
/// The winning lines are represented as arrays of tuples, where each tuple contains the coordinates of the cells in the line.
static LINES: [[(usize, usize); 3]; 8] = [
    // Rows
    [(0, 0), (1, 0), (2, 0)],
    [(0, 1), (1, 1), (2, 1)],
    [(0, 2), (1, 2), (2, 2)],
    // Columns
    [(0, 0), (0, 1), (0, 2)],
    [(1, 0), (1, 1), (1, 2)],
    [(2, 0), (2, 1), (2, 2)],
    // Diagonals
    [(0, 0), (1, 1), (2, 2)],
    [(2, 0), (1, 1), (0, 2)],
];

/// The `Player` enum represents the two players in the Tic Tac Toe game.
/// Each player is represented by a symbol: `X` or `O`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Player {
    X,
    O,
}

/// The `Cell` enum represents the state of a cell in the Tic Tac Toe game.
/// Each cell can be either `Empty` or `Occupied` by a player.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Cell {
    Empty,
    Occupied(Player),
}

/// The `Game` struct represents the state of the Tic Tac Toe game.
#[derive(Debug, Clone)]
pub struct Game {
    board: [[Cell; 3]; 3],
    current_player: Player,
    moves: Vec<(usize, usize)>,
    winning_line: Option<[(usize, usize); 3]>,
}

/// The `Game` struct represents the state of the Tic Tac Toe game.
/// It contains the game board, the current player, a list of moves made, and the winning line if any.
impl Game {
    /// Creates a new game instance with an empty board and player X starting.
    /// The board is represented as a 2D array of `Cell` enums, where each cell can be either `Empty` or `Occupied` by a player.
    pub fn new() -> Self {
        Self {
            board: [[Cell::Empty; 3]; 3],
            current_player: Player::X,
            moves: Vec::new(),
            winning_line: None,
        }
    }

    /// Resets the game state to the initial state.
    pub fn reset(&mut self) {
        // Reset the game state
        self.board = [[Cell::Empty; 3]; 3];
        self.moves.clear();
        self.winning_line = None;
        self.current_player = Player::X;
    }

    /// Makes a move for the current player at the specified coordinates (x, y).
    /// The move is only made if the game is not over and the cell is empty.
    pub fn make_move(&mut self, x: usize, y: usize) {
        // Check if the game is over or the cell is already occupied
        if self.is_over() || self.board[y][x] != Cell::Empty {
            return;
        }

        // Place the current player's symbol on the board
        self.board[y][x] = Cell::Occupied(self.current_player);
        self.moves.push((x, y));

        // Remove the oldest move if there are more than 6 moves
        if self.moves.len() > 6 {
            let (old_x, old_y) = self.moves.remove(0);
            self.board[old_y][old_x] = Cell::Empty;
        }

        // Check for a winner
        self.check_winner();

        // Switch players if the game is not over
        if !self.is_over() {
            self.switch_player();
        }
    }

    /// Switches the current player to the other player.
    /// If the current player is `X`, it switches to `O`, and vice versa.
    pub fn switch_player(&mut self) {
        self.current_player = match self.current_player {
            Player::X => Player::O,
            Player::O => Player::X,
        };
    }

    /// Checks if there is a winner by checking all possible winning lines.
    /// If a winning line is found, it sets the `winning_line` field to the coordinates of the winning line.
    pub fn check_winner(&mut self) {
        // Check all possible winning lines
        for line in &LINES {
            if let [Cell::Occupied(p1), Cell::Occupied(p2), Cell::Occupied(p3)] = [
                self.board[line[0].1][line[0].0],
                self.board[line[1].1][line[1].0],
                self.board[line[2].1][line[2].0],
            ] {
                // Check if all three cells in the line are occupied by the same player
                if p1 == p2 && p2 == p3 {
                    self.winning_line = Some(*line);
                    return;
                }
            }
        }

        // No winner found, set winning line to None
        self.winning_line = None;
    }

    /// Checks if the game is over (i.e., if there is a winner).
    pub fn is_over(&self) -> bool {
        self.winning_line.is_some()
    }

    /// Returns the current player.
    /// The current player is the one who is about to make a move.
    pub fn get_current_player(&self) -> Player {
        self.current_player
    }

    /// Returns the game board.
    /// The board is represented as a 2D array of `Cell` enums, where each cell can be either `Empty` or `Occupied` by a player.
    pub fn get_board_cell(&self, x: usize, y: usize) -> Cell {
        self.board[y][x]
    }

    /// Returns the current winning line if the game is over.
    pub fn get_winning_line(&self) -> Option<[(usize, usize); 3]> {
        self.winning_line
    }
}
