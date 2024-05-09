use crate::cell::Cell;
use crate::player::Player;
use crate::r#move::Move;

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
pub enum GameError {
    InvalidMove,
}

#[derive(Clone)]
pub struct Board {
    position: Vec<Vec<Cell>>,
    pub x: usize,
    pub y: usize,
    move_history: Vec<Move>,
}

impl Board {
    pub fn new(x: usize, y: usize) -> Self {
        Board {
            position: vec![vec![Cell::Empty; x]; y],
            x,
            y,
            move_history: vec![],
        }
    }
    pub fn get_position(&self) -> &Vec<Vec<Cell>> {
        return &self.position;
    }
    pub fn check_player_for_win(&self, player: Player) -> bool {
        return check_rows_in_position(&self.position, 3, player)
            || check_columns_in_position(&self.position, 3, player)
            || check_diags_in_position(&self.position, 3, player);
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
    pub fn make_move(&mut self, moove: Move) -> Result<(), GameError> {
        if !self.is_valid_move(&moove) {
            return Err(GameError::InvalidMove);
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

fn check_diags_in_position(position: &Vec<Vec<Cell>>, win_req: usize, player: Player) -> bool {
    for i in 0..(position.len() + 1 - win_req) {
        for j in 0..((position[i].len() + 1) / 2) {
            let mut score = 0;
            for k in 0..win_req {
                if i + k >= position.len() || j + k >= position[i].len() {
                    continue;
                }
                if position[i + k][j + k] == Cell::Filed(player) {
                    score += 1;
                    if score == win_req {
                        return true;
                    }
                    continue;
                }
                score = 0;
            }
        }
    }
    for i in 0..(position.len() + 1 - win_req) {
        for j in ((position[i].len()) / 2)..position[i].len() {
            let mut score = 0;
            for k in 0..win_req {
                if i + k >= position.len() || j < k {
                    continue;
                }
                if position[i + k][j - k] == Cell::Filed(player) {
                    score += 1;
                    if score == win_req {
                        return true;
                    }
                    continue;
                }
                score = 0;
            }
        }
    }
    return false;
}

fn check_rows_in_position(position: &Vec<Vec<Cell>>, win_req: usize, player: Player) -> bool {
    for i in 0..position.len() {
        let mut score = 0;
        for j in 0..position.len() {
            if position[i][j] != Cell::Filed(player) {
                score = 0;
                continue;
            }
            score += 1;
            if score >= win_req {
                return true;
            }
        }
    }
    return false;
}

fn check_columns_in_position(position: &Vec<Vec<Cell>>, win_req: usize, player: Player) -> bool {
    for i in 0..position.len() {
        let mut score = 0;
        for j in 0..position.len() {
            if position[j][i] != Cell::Filed(player) {
                score = 0;
                continue;
            }
            score += 1;
            if score >= win_req {
                return true;
            }
        }
    }
    return false;
}

// fn NAME + p/n/pn => positie results, negative results and both + board size
#[cfg(test)]
mod check_player_for_win_modules {
    use super::*;
    #[test]
    fn check_rows_in_position_p_4x4() {
        let p = Player::new(1, 'X');
        let positions = [
            vec![
                vec![
                    Cell::Filed(p),
                    Cell::Filed(p),
                    Cell::Filed(p),
                    Cell::Filed(p),
                ],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            ],
            vec![
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![
                    Cell::Filed(p),
                    Cell::Filed(p),
                    Cell::Filed(p),
                    Cell::Filed(p),
                ],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            ],
            vec![
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![
                    Cell::Filed(p),
                    Cell::Filed(p),
                    Cell::Filed(p),
                    Cell::Filed(p),
                ],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            ],
            vec![
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![
                    Cell::Filed(p),
                    Cell::Filed(p),
                    Cell::Filed(p),
                    Cell::Filed(p),
                ],
            ],
            vec![
                vec![Cell::Filed(p), Cell::Filed(p), Cell::Filed(p), Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            ],
            vec![
                vec![Cell::Filed(p), Cell::Filed(p), Cell::Filed(p), Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            ],
            vec![
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Filed(p), Cell::Filed(p), Cell::Filed(p), Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            ],
            vec![
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Filed(p), Cell::Filed(p), Cell::Filed(p), Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            ],
            vec![
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Filed(p), Cell::Filed(p), Cell::Filed(p), Cell::Empty],
            ],
            vec![
                vec![Cell::Empty, Cell::Filed(p), Cell::Filed(p), Cell::Filed(p)],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            ],
            vec![
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Filed(p), Cell::Filed(p), Cell::Filed(p)],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            ],
            vec![
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Filed(p), Cell::Filed(p), Cell::Filed(p)],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            ],
            vec![
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Filed(p), Cell::Filed(p), Cell::Filed(p)],
            ],
        ];
        for position in positions {
            assert!(check_rows_in_position(&position, 3, p))
        }
    }
    #[test]
    fn check_columnst_in_position_p_4x4() {
        let p = Player::new(1, 'X');
        let positions = [
            vec![
                vec![Cell::Filed(p), Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Filed(p), Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Filed(p), Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Filed(p), Cell::Empty, Cell::Empty, Cell::Empty],
            ],
            vec![
                vec![Cell::Empty, Cell::Filed(p), Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Filed(p), Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Filed(p), Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Filed(p), Cell::Empty, Cell::Empty],
            ],
            vec![
                vec![Cell::Empty, Cell::Empty, Cell::Filed(p), Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Filed(p), Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Filed(p), Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Filed(p), Cell::Empty],
            ],
            vec![
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Filed(p)],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Filed(p)],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Filed(p)],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Filed(p)],
            ],
            vec![
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Filed(p), Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Filed(p), Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Filed(p), Cell::Empty, Cell::Empty, Cell::Empty],
            ],
            vec![
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Filed(p), Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Filed(p), Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Filed(p), Cell::Empty, Cell::Empty],
            ],
            vec![
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Filed(p), Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Filed(p), Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Filed(p), Cell::Empty],
            ],
            vec![
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Filed(p)],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Filed(p)],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Filed(p)],
            ],
            vec![
                vec![Cell::Filed(p), Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Filed(p), Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Filed(p), Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            ],
            vec![
                vec![Cell::Empty, Cell::Filed(p), Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Filed(p), Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Filed(p), Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            ],
            vec![
                vec![Cell::Empty, Cell::Empty, Cell::Filed(p), Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Filed(p), Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Filed(p), Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            ],
            vec![
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Filed(p)],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Filed(p)],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Filed(p)],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            ],
        ];
        for position in positions {
            assert!(check_columns_in_position(&position, 3, p))
        }
    }
    #[test]
    fn check_diags_in_position_p_4x4() {
        let p = Player::new(1, 'X');

        //4x4
        let positions = vec![
            vec![
                vec![Cell::Filed(p), Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Filed(p), Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Filed(p), Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Filed(p)],
            ],
            vec![
                vec![Cell::Filed(p), Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Filed(p), Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Filed(p), Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            ],
            vec![
                vec![Cell::Empty, Cell::Filed(p), Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Filed(p), Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Filed(p)],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            ],
            vec![
                vec![Cell::Empty, Cell::Empty, Cell::Filed(p), Cell::Empty],
                vec![Cell::Empty, Cell::Filed(p), Cell::Empty, Cell::Empty],
                vec![Cell::Filed(p), Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            ],
            vec![
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Filed(p)],
                vec![Cell::Empty, Cell::Empty, Cell::Filed(p), Cell::Empty],
                vec![Cell::Empty, Cell::Filed(p), Cell::Empty, Cell::Empty],
                vec![Cell::Filed(p), Cell::Empty, Cell::Empty, Cell::Empty],
            ],
            vec![
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Filed(p)],
                vec![Cell::Empty, Cell::Empty, Cell::Filed(p), Cell::Empty],
                vec![Cell::Empty, Cell::Filed(p), Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            ],
            vec![
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Filed(p), Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Filed(p), Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Filed(p), Cell::Empty],
            ],
            vec![
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Filed(p), Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Filed(p), Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Filed(p)],
            ],
            vec![
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Filed(p), Cell::Empty],
                vec![Cell::Empty, Cell::Filed(p), Cell::Empty, Cell::Empty],
                vec![Cell::Filed(p), Cell::Empty, Cell::Empty, Cell::Empty],
            ],
            vec![
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Filed(p)],
                vec![Cell::Empty, Cell::Empty, Cell::Filed(p), Cell::Empty],
                vec![Cell::Empty, Cell::Filed(p), Cell::Empty, Cell::Empty],
            ],
        ];
        let mut counter = 0;
        for position in positions {
            assert!(
                check_diags_in_position(&position, 3, p),
                "position {}",
                counter
            );
            counter += 1;
        }
    }
}

#[cfg(test)]
mod make_move_tests {
    use crate::{cell::Cell, player::Player, r#move::Move};

    use super::Board;

    #[test]
    fn fill_board_p_3x3() {
        let position3x3 = vec![vec![Cell::Empty; 3]; 3];
        let p = Player::new(1, 'X');
        let mut board = Board::new(3, 3);

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
