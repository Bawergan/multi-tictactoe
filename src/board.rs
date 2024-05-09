use crate::cell::Cell;
use crate::r#move::Move;
#[derive(Clone)]
pub struct Board {
    position: Vec<Vec<Cell>>,
    pub x: usize,
    pub y: usize,
    player_count: usize,
    move_history: Vec<Move>,
}
impl Board {
    pub fn new(x: usize, y: usize, player_count: usize) -> Self {
        Board {
            position: vec![vec![Cell::Empty; x]; y],
            x,
            y,
            player_count,
            move_history: vec![],
        }
    }
    pub fn get_position(&self) -> &Vec<Vec<Cell>> {
        return &self.position;
    }
    pub fn check_player_for_win(&self, player_id: usize) -> bool {
        return check_rows_in_position(&self.position, 3, player_id)
            || check_columns_in_position(&self.position, 3, player_id)
            || check_diags_in_position(&self.position, 3, player_id);
    }
    pub fn is_valid_move(&self, coord: (usize, usize)) -> bool {
        if coord.0 >= self.y || coord.1 >= self.x {
            return false;
        }
        if self.position[coord.0][coord.1] != Cell::Empty {
            return false;
        }
        return true;
    }
    pub fn make_move(&mut self, moove: Move) {
        if moove.coord.0 >= self.y || moove.coord.1 >= self.x {
            panic!("coord out of bounds");
        }
        let mut new_position = vec![];
        for row in 0..self.position.len() {
            let mut new_row = vec![];
            for cell in 0..self.position[row].len() {
                if row == moove.coord.0 && cell == moove.coord.1 {
                    new_row.push(Cell::Filed(moove.player.id))
                } else {
                    new_row.push(self.position[row][cell])
                }
            }
            new_position.push(new_row);
        }
        self.move_history.push(moove);
        self.position = new_position;
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

fn check_rows_in_position(position: &Vec<Vec<Cell>>, win_req: usize, player_id: usize) -> bool {
    for i in 0..position.len() {
        let mut score = 0;
        for j in 0..position.len() {
            if position[i][j] != Cell::Filed(player_id) {
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

fn check_columns_in_position(position: &Vec<Vec<Cell>>, win_req: usize, player_id: usize) -> bool {
    for i in 0..position.len() {
        let mut score = 0;
        for j in 0..position.len() {
            if position[j][i] != Cell::Filed(player_id) {
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

fn check_diags_in_position(position: &Vec<Vec<Cell>>, win_req: usize, player_id: usize) -> bool {
    let positions = devide_to_min_dim(position, win_req);
    for position in positions {
        if check_main_diags(&position, player_id) {
            return true;
        }
    }
    return false;
}

fn check_main_diags(position: &Vec<Vec<Cell>>, player_id: usize) -> bool {
    let mut first = true;
    let mut second: bool = true;
    for i in 0..position.len() {
        for j in 0..position[i].len() {
            if i == j {
                if position[i][j] != Cell::Filed(player_id) {
                    first = false;
                }
            }
            if i == (position[i].len() - j - 1) {
                if position[i][j] != Cell::Filed(player_id) {
                    second = false;
                }
            }
        }
    }
    return first || second;
}

fn devide_to_min_dim(position: &Vec<Vec<Cell>>, min_dim: usize) -> Vec<Vec<Vec<Cell>>> {
    let mut devided_positions = vec![position.to_owned()];

    while devided_positions[0].len() > min_dim {
        let mut new_devided_positions = vec![];
        for position in &devided_positions {
            new_devided_positions.append(&mut devide_position(position))
        }

        devided_positions.clear();
        devided_positions.append(&mut new_devided_positions);
    }
    return devided_positions;
}

fn devide_position(position: &Vec<Vec<Cell>>) -> Vec<Vec<Vec<Cell>>> {
    let mut first = position[0..(position.len() - 1)].to_vec();
    for i in 0..first.len() {
        let a = first[i].len();
        first[i].remove(a - 1);
    }
    //0000    0001
    //0000 => 0001
    //0000    0001
    //0000    1111
    let mut second = position[0..(position.len() - 1)].to_vec();
    for i in 0..second.len() {
        second[i].remove(0);
    }
    //0000    1000
    //0000 => 1000
    //0000    1000
    //0000    1111
    let mut third = position[1..position.len()].to_vec();
    for i in 0..third.len() {
        let a = third[i].len();
        third[i].remove(a - 1);
    }
    //0000    1111
    //0000 => 0001
    //0000    0001
    //0000    0001
    let mut fourth = position[1..position.len()].to_vec();
    for i in 0..fourth.len() {
        fourth[i].remove(0);
    }
    //0000    1111
    //0000 => 1000
    //0000    1000
    //0000    1000
    return vec![first, second, third, fourth];
}

#[cfg(test)]
mod position {
    use super::*;
    #[test]
    fn check_rows_in_position_fn_test_only_positive() {
        let positions = [
            vec![
                vec![
                    Cell::Filed(1),
                    Cell::Filed(1),
                    Cell::Filed(1),
                    Cell::Filed(1),
                ],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            ],
            vec![
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![
                    Cell::Filed(1),
                    Cell::Filed(1),
                    Cell::Filed(1),
                    Cell::Filed(1),
                ],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            ],
            vec![
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![
                    Cell::Filed(1),
                    Cell::Filed(1),
                    Cell::Filed(1),
                    Cell::Filed(1),
                ],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            ],
            vec![
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![
                    Cell::Filed(1),
                    Cell::Filed(1),
                    Cell::Filed(1),
                    Cell::Filed(1),
                ],
            ],
            vec![
                vec![
                    Cell::Filed(1),
                    Cell::Filed(1),
                    Cell::Filed(1),
                    Cell::Filed(0),
                ],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            ],
            vec![
                vec![
                    Cell::Filed(1),
                    Cell::Filed(1),
                    Cell::Filed(1),
                    Cell::Filed(0),
                ],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            ],
            vec![
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![
                    Cell::Filed(1),
                    Cell::Filed(1),
                    Cell::Filed(1),
                    Cell::Filed(0),
                ],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            ],
            vec![
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![
                    Cell::Filed(1),
                    Cell::Filed(1),
                    Cell::Filed(1),
                    Cell::Filed(0),
                ],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            ],
            vec![
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![
                    Cell::Filed(1),
                    Cell::Filed(1),
                    Cell::Filed(1),
                    Cell::Filed(0),
                ],
            ],
            vec![
                vec![
                    Cell::Filed(0),
                    Cell::Filed(1),
                    Cell::Filed(1),
                    Cell::Filed(1),
                ],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            ],
            vec![
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![
                    Cell::Filed(0),
                    Cell::Filed(1),
                    Cell::Filed(1),
                    Cell::Filed(1),
                ],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            ],
            vec![
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![
                    Cell::Filed(0),
                    Cell::Filed(1),
                    Cell::Filed(1),
                    Cell::Filed(1),
                ],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            ],
            vec![
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![
                    Cell::Filed(0),
                    Cell::Filed(1),
                    Cell::Filed(1),
                    Cell::Filed(1),
                ],
            ],
        ];
        for position in positions {
            assert!(check_rows_in_position(&position, 3, 1))
        }
    }
    #[test]
    fn check_columnst_in_position_fn_test_only_positive() {
        let positions = [
            vec![
                vec![Cell::Filed(1), Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Filed(1), Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Filed(1), Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Filed(1), Cell::Empty, Cell::Empty, Cell::Empty],
            ],
            vec![
                vec![Cell::Empty, Cell::Filed(1), Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Filed(1), Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Filed(1), Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Filed(1), Cell::Empty, Cell::Empty],
            ],
            vec![
                vec![Cell::Empty, Cell::Empty, Cell::Filed(1), Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Filed(1), Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Filed(1), Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Filed(1), Cell::Empty],
            ],
            vec![
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Filed(1)],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Filed(1)],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Filed(1)],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Filed(1)],
            ],
            vec![
                vec![Cell::Filed(0), Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Filed(1), Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Filed(1), Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Filed(1), Cell::Empty, Cell::Empty, Cell::Empty],
            ],
            vec![
                vec![Cell::Empty, Cell::Filed(0), Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Filed(1), Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Filed(1), Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Filed(1), Cell::Empty, Cell::Empty],
            ],
            vec![
                vec![Cell::Empty, Cell::Empty, Cell::Filed(0), Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Filed(1), Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Filed(1), Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Filed(1), Cell::Empty],
            ],
            vec![
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Filed(0)],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Filed(1)],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Filed(1)],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Filed(1)],
            ],
            vec![
                vec![Cell::Filed(1), Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Filed(1), Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Filed(1), Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Filed(0), Cell::Empty, Cell::Empty, Cell::Empty],
            ],
            vec![
                vec![Cell::Empty, Cell::Filed(1), Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Filed(1), Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Filed(1), Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Filed(0), Cell::Empty, Cell::Empty],
            ],
            vec![
                vec![Cell::Empty, Cell::Empty, Cell::Filed(1), Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Filed(1), Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Filed(1), Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Filed(0), Cell::Empty],
            ],
            vec![
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Filed(1)],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Filed(1)],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Filed(1)],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Filed(0)],
            ],
        ];
        for position in positions {
            assert!(check_columns_in_position(&position, 3, 1))
        }
    }
    #[test]
    fn devide_position_fn_testing() {
        let empty_position = vec![vec![Cell::Empty; 3]; 3];
        let positions = vec![
            vec![
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Filed(1)],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Filed(1)],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Filed(1)],
                vec![
                    Cell::Filed(1),
                    Cell::Filed(1),
                    Cell::Filed(1),
                    Cell::Filed(1),
                ],
            ],
            vec![
                vec![Cell::Filed(1), Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Filed(1), Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Filed(1), Cell::Empty, Cell::Empty, Cell::Empty],
                vec![
                    Cell::Filed(1),
                    Cell::Filed(1),
                    Cell::Filed(1),
                    Cell::Filed(1),
                ],
            ],
            vec![
                vec![
                    Cell::Filed(1),
                    Cell::Filed(1),
                    Cell::Filed(1),
                    Cell::Filed(1),
                ],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Filed(1)],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Filed(1)],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Filed(1)],
            ],
            vec![
                vec![
                    Cell::Filed(1),
                    Cell::Filed(1),
                    Cell::Filed(1),
                    Cell::Filed(1),
                ],
                vec![Cell::Filed(1), Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Filed(1), Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Filed(1), Cell::Empty, Cell::Empty, Cell::Empty],
            ],
        ];
        for id in 0..positions.len() {
            assert_eq!(devide_position(&positions[id])[id], empty_position)
        }
    }
    #[test]
    fn check_main_diags_fn_testing_only_positive() {
        //4x4
        let positions = vec![
            vec![
                vec![Cell::Filed(1), Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Filed(1), Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Filed(1), Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Filed(1)],
            ],
            vec![
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Filed(1)],
                vec![Cell::Empty, Cell::Empty, Cell::Filed(1), Cell::Empty],
                vec![Cell::Empty, Cell::Filed(1), Cell::Empty, Cell::Empty],
                vec![Cell::Filed(1), Cell::Empty, Cell::Empty, Cell::Empty],
            ],
        ];
        for position in positions {
            assert!(check_main_diags(&position, 1))
        }
    }
    #[test]
    fn check_diags_in_position_fn_testing_only_positive() {
        //4x4
        let positions = vec![
            vec![
                vec![Cell::Filed(1), Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Filed(1), Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Filed(1), Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Filed(1)],
            ],
            vec![
                vec![Cell::Filed(1), Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Filed(1), Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Filed(1), Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            ],
            vec![
                vec![Cell::Empty, Cell::Filed(1), Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Filed(1), Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Filed(1)],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            ],
            vec![
                vec![Cell::Empty, Cell::Empty, Cell::Filed(1), Cell::Empty],
                vec![Cell::Empty, Cell::Filed(1), Cell::Empty, Cell::Empty],
                vec![Cell::Filed(1), Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            ],
            vec![
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Filed(1)],
                vec![Cell::Empty, Cell::Empty, Cell::Filed(1), Cell::Empty],
                vec![Cell::Empty, Cell::Filed(1), Cell::Empty, Cell::Empty],
                vec![Cell::Filed(1), Cell::Empty, Cell::Empty, Cell::Empty],
            ],
            vec![
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Filed(1)],
                vec![Cell::Empty, Cell::Empty, Cell::Filed(1), Cell::Empty],
                vec![Cell::Empty, Cell::Filed(1), Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            ],
            vec![
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Filed(1), Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Filed(1), Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Filed(1), Cell::Empty],
            ],
            vec![
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Filed(1), Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Filed(1), Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Filed(1)],
            ],
            vec![
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Filed(1), Cell::Empty],
                vec![Cell::Empty, Cell::Filed(1), Cell::Empty, Cell::Empty],
                vec![Cell::Filed(1), Cell::Empty, Cell::Empty, Cell::Empty],
            ],
            vec![
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Filed(1)],
                vec![Cell::Empty, Cell::Empty, Cell::Filed(1), Cell::Empty],
                vec![Cell::Empty, Cell::Filed(1), Cell::Empty, Cell::Empty],
            ],
        ];
        for position in positions {
            assert!(check_diags_in_position(&position, 3, 1));
        }
    }
}

#[cfg(test)]
mod board {
    use crate::{cell::Cell, player::Player, r#move::Move};

    use super::Board;

    #[test]
    fn make_move_fn_tets() {
        let position3x3 = vec![vec![Cell::Empty; 3]; 3];
        let mut board = Board::new(3, 3, 2);

        let mut desired_pos = position3x3;
        for i in 0..3 {
            for j in 0..3 {
                board.make_move(Move::new((i, j), Player::new(1, 'X'), 0));
                desired_pos[i][j] = Cell::Filed(1);

                assert_eq!(board.position, desired_pos);
            }
        }
    }
}
