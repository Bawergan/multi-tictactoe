use crate::{cell::Cell, player::Player};
use once_cell::sync::Lazy;

//this file will be a mess for now.
pub fn check_every_element(position: &Vec<Vec<Cell>>, win_req: usize, player: Player) -> bool {
    let max_len = position.len().max(position[0].len());
    for i in 0..max_len {
        let mut score1 = 0;
        let mut score2 = 0;
        for j in 0..max_len {
            if !(i < position.len() && j < position[i].len()) {
            } else if position[i][j] == Cell::Filed(player) {
                score1 += 1;
                if score1 >= win_req {
                    return true;
                }
            } else {
                score1 = 0;
            }
            if !(j < position.len() && i < position[j].len()) {
            } else if position[j][i] == Cell::Filed(player) {
                score2 += 1;
                if score2 >= win_req {
                    return true;
                };
            } else {
                score2 = 0;
            }
            let mut score3 = 0;
            let mut score4 = 0;
            if i < position.len() {
                for k in 0..win_req {
                    if i + k >= position.len() || j + k >= position[i].len() {
                    } else if position[i + k][j + k] == Cell::Filed(player) {
                        score3 += 1;
                        if score3 == win_req {
                            return true;
                        }
                    } else {
                        score3 = 0;
                    }

                    if i + k >= position.len() || j < k {
                    } else if j - k >= position[i + k].len() {
                    } else if position[i + k][j - k] == Cell::Filed(player) {
                        score4 += 1;
                        if score4 == win_req {
                            return true;
                        }
                    } else {
                        score4 = 0;
                    }
                }
            }
        }
    }
    return false;
}
pub fn check_diags_in_position(position: &Vec<Vec<Cell>>, win_req: usize, player: Player) -> bool {
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

pub fn check_rows_in_position(position: &Vec<Vec<Cell>>, win_req: usize, player: Player) -> bool {
    for i in 0..position.len() {
        let mut score = 0;
        for j in 0..position.len() {
            if position[i][j] != Cell::Filed(player) {
                score = 0;
                // continue;
            } else {
                score += 1;
                if score >= win_req {
                    return true;
                }
            }
        }
    }
    return false;
}

pub fn check_columns_in_position(
    position: &Vec<Vec<Cell>>,
    win_req: usize,
    player: Player,
) -> bool {
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

// fn NAME +
//eefn/eefp - elliminates every false negative/false positive result +
//board size +
#[cfg(test)]
mod test {
    use crate::{
        board::{self, Board},
        cell::Cell,
    };

    use super::*;
    #[test]
    fn check_rows_in_position_eefn_4x4() {
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
    fn check_columnst_in_position_eefn_4x4() {
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
    fn check_diags_in_position_eefn_4x4() {
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
    #[test]
    fn check_every_position_eefn_4x4_3() {
        let mut counter = 0;
        for position in POSITIONS.clone() {
            assert!(
                check_every_element(&position, 3, P.clone()),
                "position {}",
                counter
            );
            counter += 1;
        }
    }
    #[test]
    fn check_positions_fn_4x5_4x10_3() {
        let empty_vec = vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty];
        let mut counter = 0;
        for mut position in POSITIONS.clone() {
            for _i in 0..6 {
                position.push(empty_vec.clone());
                assert!(
                    check_every_element(&position, 3, P.clone()),
                    "position {}",
                    counter / 6
                );
                counter += 1;
            }
        }
    }
    #[test]
    fn check_positions_fn_5x4_10x4_3() {
        let mut counter = 0;
        for mut position in POSITIONS.clone() {
            for _i in 0..6 {
                for j in 0..position.len() {
                    position[j].push(Cell::Empty);
                }
                assert!(
                    check_every_element(&position, 3, P.clone()),
                    "position {}",
                    counter / 6
                );
                counter += 1;
            }
        }
    }
    #[test]
    fn check_positions_fp_3x3_10x10() {
        let upto = (10, 10);
        for win_req in 3..5 {
            for l in 0..upto.1 {
                for k in 0..upto.0 {
                    let mut position = vec![];
                    for i in 0..upto.0 {
                        let mut new_row = vec![];
                        for j in 0..upto.1 {
                            if (j + k) % win_req == 0 || (i + l) % win_req == 0 {
                                new_row.push(Cell::Empty);
                            } else {
                                new_row.push(Cell::Empty)
                            }
                        }
                        position.push(new_row);
                    }
                    assert!(
                        !check_every_element(&position, win_req, *P),
                        "position: {}",
                        Board::new(position.len(), position[0].len(), 3)
                    )
                }
            }
        }
    }
}
static P: Lazy<Player> = Lazy::new(|| Player::new(1, 'X'));

static POSITIONS: Lazy<Vec<Vec<Vec<Cell>>>> = Lazy::new(|| {
    vec![
        //rows = 12
        vec![
            vec![
                Cell::Filed(*P),
                Cell::Filed(*P),
                Cell::Filed(*P),
                Cell::Filed(*P),
            ],
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
        ],
        vec![
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            vec![
                Cell::Filed(*P),
                Cell::Filed(*P),
                Cell::Filed(*P),
                Cell::Filed(*P),
            ],
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
        ],
        vec![
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            vec![
                Cell::Filed(*P),
                Cell::Filed(*P),
                Cell::Filed(*P),
                Cell::Filed(*P),
            ],
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
        ],
        vec![
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            vec![
                Cell::Filed(*P),
                Cell::Filed(*P),
                Cell::Filed(*P),
                Cell::Filed(*P),
            ],
        ],
        vec![
            vec![
                Cell::Filed(*P),
                Cell::Filed(*P),
                Cell::Filed(*P),
                Cell::Empty,
            ],
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
        ],
        vec![
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            vec![
                Cell::Filed(*P),
                Cell::Filed(*P),
                Cell::Filed(*P),
                Cell::Empty,
            ],
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
        ],
        vec![
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            vec![
                Cell::Filed(*P),
                Cell::Filed(*P),
                Cell::Filed(*P),
                Cell::Empty,
            ],
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
        ],
        vec![
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            vec![
                Cell::Filed(*P),
                Cell::Filed(*P),
                Cell::Filed(*P),
                Cell::Empty,
            ],
        ],
        vec![
            vec![
                Cell::Empty,
                Cell::Filed(*P),
                Cell::Filed(*P),
                Cell::Filed(*P),
            ],
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
        ],
        vec![
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            vec![
                Cell::Empty,
                Cell::Filed(*P),
                Cell::Filed(*P),
                Cell::Filed(*P),
            ],
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
        ],
        vec![
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            vec![
                Cell::Empty,
                Cell::Filed(*P),
                Cell::Filed(*P),
                Cell::Filed(*P),
            ],
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
        ],
        vec![
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            vec![
                Cell::Empty,
                Cell::Filed(*P),
                Cell::Filed(*P),
                Cell::Filed(*P),
            ],
        ],
        //columns = 12
        vec![
            vec![Cell::Filed(*P), Cell::Empty, Cell::Empty, Cell::Empty],
            vec![Cell::Filed(*P), Cell::Empty, Cell::Empty, Cell::Empty],
            vec![Cell::Filed(*P), Cell::Empty, Cell::Empty, Cell::Empty],
            vec![Cell::Filed(*P), Cell::Empty, Cell::Empty, Cell::Empty],
        ],
        vec![
            vec![Cell::Empty, Cell::Filed(*P), Cell::Empty, Cell::Empty],
            vec![Cell::Empty, Cell::Filed(*P), Cell::Empty, Cell::Empty],
            vec![Cell::Empty, Cell::Filed(*P), Cell::Empty, Cell::Empty],
            vec![Cell::Empty, Cell::Filed(*P), Cell::Empty, Cell::Empty],
        ],
        vec![
            vec![Cell::Empty, Cell::Empty, Cell::Filed(*P), Cell::Empty],
            vec![Cell::Empty, Cell::Empty, Cell::Filed(*P), Cell::Empty],
            vec![Cell::Empty, Cell::Empty, Cell::Filed(*P), Cell::Empty],
            vec![Cell::Empty, Cell::Empty, Cell::Filed(*P), Cell::Empty],
        ],
        vec![
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Filed(*P)],
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Filed(*P)],
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Filed(*P)],
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Filed(*P)],
        ],
        vec![
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            vec![Cell::Filed(*P), Cell::Empty, Cell::Empty, Cell::Empty],
            vec![Cell::Filed(*P), Cell::Empty, Cell::Empty, Cell::Empty],
            vec![Cell::Filed(*P), Cell::Empty, Cell::Empty, Cell::Empty],
        ],
        vec![
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            vec![Cell::Empty, Cell::Filed(*P), Cell::Empty, Cell::Empty],
            vec![Cell::Empty, Cell::Filed(*P), Cell::Empty, Cell::Empty],
            vec![Cell::Empty, Cell::Filed(*P), Cell::Empty, Cell::Empty],
        ],
        vec![
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            vec![Cell::Empty, Cell::Empty, Cell::Filed(*P), Cell::Empty],
            vec![Cell::Empty, Cell::Empty, Cell::Filed(*P), Cell::Empty],
            vec![Cell::Empty, Cell::Empty, Cell::Filed(*P), Cell::Empty],
        ],
        vec![
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Filed(*P)],
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Filed(*P)],
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Filed(*P)],
        ],
        vec![
            vec![Cell::Filed(*P), Cell::Empty, Cell::Empty, Cell::Empty],
            vec![Cell::Filed(*P), Cell::Empty, Cell::Empty, Cell::Empty],
            vec![Cell::Filed(*P), Cell::Empty, Cell::Empty, Cell::Empty],
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
        ],
        vec![
            vec![Cell::Empty, Cell::Filed(*P), Cell::Empty, Cell::Empty],
            vec![Cell::Empty, Cell::Filed(*P), Cell::Empty, Cell::Empty],
            vec![Cell::Empty, Cell::Filed(*P), Cell::Empty, Cell::Empty],
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
        ],
        vec![
            vec![Cell::Empty, Cell::Empty, Cell::Filed(*P), Cell::Empty],
            vec![Cell::Empty, Cell::Empty, Cell::Filed(*P), Cell::Empty],
            vec![Cell::Empty, Cell::Empty, Cell::Filed(*P), Cell::Empty],
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
        ],
        vec![
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Filed(*P)],
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Filed(*P)],
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Filed(*P)],
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
        ],
        //diagonals = 10
        vec![
            vec![Cell::Filed(*P), Cell::Empty, Cell::Empty, Cell::Empty],
            vec![Cell::Empty, Cell::Filed(*P), Cell::Empty, Cell::Empty],
            vec![Cell::Empty, Cell::Empty, Cell::Filed(*P), Cell::Empty],
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Filed(*P)],
        ],
        vec![
            vec![Cell::Filed(*P), Cell::Empty, Cell::Empty, Cell::Empty],
            vec![Cell::Empty, Cell::Filed(*P), Cell::Empty, Cell::Empty],
            vec![Cell::Empty, Cell::Empty, Cell::Filed(*P), Cell::Empty],
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
        ],
        vec![
            vec![Cell::Empty, Cell::Filed(*P), Cell::Empty, Cell::Empty],
            vec![Cell::Empty, Cell::Empty, Cell::Filed(*P), Cell::Empty],
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Filed(*P)],
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
        ],
        vec![
            vec![Cell::Empty, Cell::Empty, Cell::Filed(*P), Cell::Empty],
            vec![Cell::Empty, Cell::Filed(*P), Cell::Empty, Cell::Empty],
            vec![Cell::Filed(*P), Cell::Empty, Cell::Empty, Cell::Empty],
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
        ],
        vec![
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Filed(*P)],
            vec![Cell::Empty, Cell::Empty, Cell::Filed(*P), Cell::Empty],
            vec![Cell::Empty, Cell::Filed(*P), Cell::Empty, Cell::Empty],
            vec![Cell::Filed(*P), Cell::Empty, Cell::Empty, Cell::Empty],
        ],
        vec![
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Filed(*P)],
            vec![Cell::Empty, Cell::Empty, Cell::Filed(*P), Cell::Empty],
            vec![Cell::Empty, Cell::Filed(*P), Cell::Empty, Cell::Empty],
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
        ],
        vec![
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            vec![Cell::Filed(*P), Cell::Empty, Cell::Empty, Cell::Empty],
            vec![Cell::Empty, Cell::Filed(*P), Cell::Empty, Cell::Empty],
            vec![Cell::Empty, Cell::Empty, Cell::Filed(*P), Cell::Empty],
        ],
        vec![
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            vec![Cell::Empty, Cell::Filed(*P), Cell::Empty, Cell::Empty],
            vec![Cell::Empty, Cell::Empty, Cell::Filed(*P), Cell::Empty],
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Filed(*P)],
        ],
        vec![
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            vec![Cell::Empty, Cell::Empty, Cell::Filed(*P), Cell::Empty],
            vec![Cell::Empty, Cell::Filed(*P), Cell::Empty, Cell::Empty],
            vec![Cell::Filed(*P), Cell::Empty, Cell::Empty, Cell::Empty],
        ],
        vec![
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Filed(*P)],
            vec![Cell::Empty, Cell::Empty, Cell::Filed(*P), Cell::Empty],
            vec![Cell::Empty, Cell::Filed(*P), Cell::Empty, Cell::Empty],
        ],
    ]
});
