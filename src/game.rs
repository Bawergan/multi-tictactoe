use crate::board::Board;
use crate::cell::Cell;
use crate::player::Player;
use crate::r#move::Move;
#[derive(Debug)]
pub enum GameError {
    // unable to parse move for current turn
    InvalidMove,
}
pub enum WinDraw {
    Win,
    Draw,
    Noting,
}
#[derive(Clone)]
pub struct Game {
    board: Board,
    players: Vec<Player>,
}
impl Game {
    pub fn new(board_size: (usize, usize), players: &Vec<Player>) -> Self {
        Game {
            board: Board::new(board_size.0, board_size.1, players.len()),
            players: players.to_vec(),
        }
    }
    pub fn draw_board(&self) {
        let mut board = vec![vec!['*'; self.board.x]; self.board.y];
        let position = self.board.get_position();
        for i in 0..board.len() {
            for j in 0..board[i].len() {
                match position[i][j] {
                    Cell::Empty => continue,
                    Cell::Filed(player) => {
                        board[i][j] = self.players.iter().find(|f| f.id == player).unwrap().skin
                    }
                }
            }
        }
        for i in 0..board.len() {
            for j in 0..board[i].len() {
                print!("{}", board[i][j]);
            }
            println!();
        }
    }
    pub fn make_move(
        &mut self,
        coord: (usize, usize),
        player: Player,
        move_number: usize,
    ) -> Result<(), GameError> {
        if self.board.is_valid_move(coord) {
            self.board.make_move(Move::new(coord, player, move_number));
            return Ok(());
        }
        Err(GameError::InvalidMove)
    }
    pub fn game_win_draw_checker(&self, player_id: usize) -> WinDraw {
        if self.board.check_player_for_win(player_id) {
            return WinDraw::Win;
        }
        if self.board.check_position_for_draw() {
            return WinDraw::Draw;
        }
        return WinDraw::Noting;
    }
    pub fn get_possible_moves_coord(&self) -> Vec<(usize, usize)> {
        let mut moves = vec![];
        for i in 0..self.board.y {
            for j in 0..self.board.x {
                if self.board.is_valid_move((i, j)) {
                    moves.push((i, j))
                }
            }
        }
        return moves;
    }
}
