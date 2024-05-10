use crate::cell::Cell;
use crate::player::Player;
use crate::r#move::Move;

use crate::board::board_checkers::*;
mod board_checkers;

impl core::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut board = vec![vec!['*'; self.x]; self.y];
        let position = &self.position;
        for i in 0..board.len() {
            for j in 0..board[i].len() {
                match position[i][j] {
                    Cell::Empty => continue,
                    Cell::Filed(player) => board[i][j] = player.skin,
                }
            }
        }
        let a = (0..(self.x))
            .collect::<Vec<_>>()
            .iter()
            .map(|v| v.to_string())
            .collect::<String>();
        writeln!(f, "  {}", a)?;
        for i in 0..board.len() {
            write!(f, "{i} ")?;
            for j in 0..board[i].len() {
                write!(f, "{}", board[i][j])?;
            }
            writeln!(f)?;
        }
        write!(f, "")
    }
}
#[derive(Debug)]
pub enum BoardError {
    InvalidMove,
}

#[derive(Clone)]
pub struct Board {
    position: Vec<Vec<Cell>>,
    pub x: usize,
    pub y: usize,
    move_history: Vec<Move>,
    win_req: usize
}

impl Board {
    pub fn new(x: usize, y: usize, win_req: usize) -> Self {
        Board {
            position: vec![vec![Cell::Empty; x]; y],
            x,
            y,
            move_history: vec![],
            win_req
        }
    }
    pub fn get_position(&self) -> &Vec<Vec<Cell>> {
        return &self.position;
    }
    pub fn check_player_for_win(&self, player: Player) -> bool {
        // return check_rows_in_position(&self.position, 3, player)
        //     || check_columns_in_position(&self.position, 3, player)
        //     || check_diags_in_position(&self.position, 3, player);
        check_every_element(&self.position, self.win_req, player)
    }
    pub fn is_valid_move(&self, moove: &Move) -> bool {
        if moove.coord.0 >= self.y || moove.coord.1 >= self.x {
            return false;
        }
        if self.position[moove.coord.0][moove.coord.1] != Cell::Empty {
            return false;
        }
        return true;
    }
    pub fn make_move(&mut self, moove: Move) -> Result<(), BoardError> {
        if !self.is_valid_move(&moove) {
            return Err(BoardError::InvalidMove);
        }

        let mut new_position = vec![];
        for row in 0..self.position.len() {
            let mut new_row = vec![];
            for cell in 0..self.position[row].len() {
                if row == moove.coord.0 && cell == moove.coord.1 {
                    new_row.push(Cell::Filed(moove.player))
                } else {
                    new_row.push(self.position[row][cell])
                }
            }
            new_position.push(new_row);
        }

        self.move_history.push(moove);
        self.position = new_position;
        Ok(())
    }
    pub fn get_empty_cells_coords(&self) -> Vec<(usize, usize)> {
        let mut empty_cells_coords = vec![];
        for i in 0..self.y {
            for j in 0..self.x {
                if self.position[i][j] == Cell::Empty {
                    empty_cells_coords.push((i, j))
                }
            }
        }
        return empty_cells_coords;
    }
    pub fn check_position_for_draw(&self) -> bool {
        for row in &self.position {
            for cell in row {
                if cell == &Cell::Empty {
                    return false;
                }
            }
        }
        return true;
    }
}

#[cfg(test)]
mod make_move_tests {
    use crate::{cell::Cell, player::Player, r#move::Move};

    use super::Board;

    #[test]
    fn fill_board_fn_3x3() {
        let position3x3 = vec![vec![Cell::Empty; 3]; 3];
        let p = Player::new(1, 'X');
        let mut board = Board::new(3, 3, 3);

        let mut desired_pos = position3x3;
        for i in 0..3 {
            for j in 0..3 {
                board
                    .make_move(Move::new((i, j), Player::new(1, 'X'), 0))
                    .expect("invalid move shoud be valid");
                desired_pos[i][j] = Cell::Filed(p);

                assert_eq!(board.position, desired_pos);
            }
        }
    }
}
#[cfg(test)]
mod other {
    use crate::{board::P, cell::Cell, r#move::Move, player::Player};

    use super::Board;

    #[test]
    fn check_position_for_draw_eefn_eefp_every_pos(){
        for x in 3..10{
            for y in 3..10{
                let mut board = Board::new(x, y, 3);
                for i in 0..board.y{
                    for j in 0..board.x{
                        _ = board.make_move(Move::new((i,j), Player::new(i+j, 'X'), i+j))
                    }
                }
                assert!(board.check_position_for_draw(), "position: {}", board)
            }
        }
        for x in 3..10{
            for y in 3..10{
                let mut board = Board::new(x, y, 3);
                for i in 0..board.y{
                    for j in 0..board.x -1{
                        _ = board.make_move(Move::new((i,j), Player::new(i+j, 'X'), i+j))
                    }
                }
                assert!(!board.check_position_for_draw(), "position: {}", board)
            }
        }
    }
    #[test]
    fn is_valid_move_fp_fn_3x3(){
        let mut board = Board::new(3, 3, 3);
        assert!(!board.is_valid_move(&Move::new((3,0), *P, 0)));
        assert!(!board.is_valid_move(&Move::new((0,3), *P, 0)));
        assert!(board.is_valid_move(&Move::new((0,0), *P, 0)));
        _ = board.make_move(Move::new((0,0), *P, 0));
        assert!(!board.is_valid_move(&Move::new((0,0), *P, 0)));
    }
    #[test]
    fn check_player_for_win_fp_fn_3x3(){
        let mut board = Board::new(3, 3, 3);

        assert!(!board.check_player_for_win(*P));
        _ = board.make_move(Move::new((0,0), *P, 0));
        _ = board.make_move(Move::new((0,1), *P, 1));
        _ = board.make_move(Move::new((0,2), *P, 2));
        assert!(board.check_player_for_win(*P))
    }
    #[test]
    fn get_empty_cells_coords_fn_fp(){
        let pos = vec![vec![Cell::Empty;3];3];
        let mut empty_cells = vec![];
        for i in 0..pos.len(){
            for j in 0..pos[i].len(){
                if pos[i][j] == Cell::Empty{
                    empty_cells.push((i,j))
                }
            }
        }
        let mut board = Board::new(3, 3, 3);
        assert_eq!(board.get_empty_cells_coords(), empty_cells);
        _ = board.make_move(Move::new((0,1), *P, 0));
        assert_ne!(board.get_empty_cells_coords(), empty_cells)
    }
    #[test]
    fn get_position_fn_fp_3x3(){
        let pos = vec![vec![Cell::Empty;3];3];
        let mut board = Board::new(3, 3, 3);
        assert_eq!(board.get_position(), &pos);
        _ = board.make_move(Move::new((0,1), *P, 0));
        assert_ne!(board.get_position(), &pos);
    }
}
use once_cell::sync::Lazy;
static P: Lazy<Player> = Lazy::new(|| Player::new(1, 'X'));
