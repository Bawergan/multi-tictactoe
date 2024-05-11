use crate::{board::Board, r#move::Move};
use rand::seq::SliceRandom;

pub fn ask_bot(board: Board) -> String {
    let moove = match board.get_current_player().p_type {
        crate::player::PlayerType::Bot => deeper(board),
        crate::player::PlayerType::Terminal => return "asas".to_string(),
        crate::player::PlayerType::JankyBot => janky_bot(board),
        crate::player::PlayerType::Martin => martin(board),
        crate::player::PlayerType::Deeper => deeper(board),
        crate::player::PlayerType::Deeperer => deeperer(board),
        crate::player::PlayerType::Deeperest => deeperest(board),
    };

    return "mk ".to_string() + moove.1.to_string().as_str() + " " + moove.0.to_string().as_str();
}

fn wining_bot(mut board: Board) -> (usize, usize) {
    match board.make_move(Move::new((0, 0), board.get_current_player())) {
        Ok(_) => return (0, 0),
        Err(_) => {}
    };
    match board.make_move(Move::new((0, 1), board.get_current_player())) {
        Ok(_) => return (0, 1),
        Err(_) => {}
    };
    match board.make_move(Move::new((0, 2), board.get_current_player())) {
        Ok(_) => return (0, 2),
        Err(_) => {}
    };
    return (2, 2);
}
//plays random moves
fn janky_bot(board: Board) -> (usize, usize) {
    return board
        .get_empty_cells_coords()
        .choose(&mut rand::thread_rng())
        .unwrap()
        .to_owned();
}
fn martin(mut board: Board) -> (usize, usize) {
    let m = board.get_empty_cells_coords()[0];
    for m in board.get_empty_cells_coords() {
        _ = board.make_move(Move::new(m, board.get_current_player()));
        if board.check_prev_player_for_win() {
            return m;
        }
        _ = board.undo_move();
    }
    return m;
}
fn deeper(mut board: Board) -> (usize, usize) {
    for m in board.get_empty_cells_coords() {
        _ = board.make_move(Move::new(m, board.get_current_player()));
        //if you'll win, win
        if board.check_prev_player_for_win() {
            return m;
        }

        for mm in board.get_empty_cells_coords() {
            _ = board.make_move(Move::new(mm, board.get_current_player()));
            //if oponent'll win, do not allow it
            if board.check_prev_player_for_win() {
                return mm;
            }
            _ = board.undo_move();
        }

        _ = board.undo_move();
    }

    //or just play the first one
    return board.get_empty_cells_coords()[0];
}
fn deeperer(mut board: Board) -> (usize, usize) {
    let mut first = None;
    let mut second = None;
    let mut third = None;
    for m in board.get_empty_cells_coords() {
        _ = board.make_move(Move::new(m, board.get_current_player()));
        //if i'll win, i'll win
        if board.check_prev_player_for_win() {
            first = Some(m);
        }

        for mm in board.get_empty_cells_coords() {
            _ = board.make_move(Move::new(mm, board.get_current_player()));
            //if oponent'll win, do not allow it
            if board.check_prev_player_for_win() {
                second = Some(mm);
            }

            for mmm in board.get_empty_cells_coords() {
                _ = board.make_move(Move::new(mmm, board.get_current_player()));
                //if oponent'll win, do not allow it
                //OR if i'll win, i'll win
                if board.check_prev_player_for_win() {
                    third = Some(mmm);
                }
                _ = board.undo_move();
            }
            _ = board.undo_move();
        }

        _ = board.undo_move();
    }
    // println!("{:?}", vec![first, second, third]);
    //or just play the first one
    return first.unwrap_or(second.unwrap_or(third.unwrap_or(board.get_empty_cells_coords()[0])));
}
fn deeperest_recursion(
    mut board: Board,
    depth: usize,
    max_depth: usize,
    candidates: &mut [Option<(usize, usize)>; DEEPEREST_MAX_DEPTH],
) {
    if depth >= max_depth {
        return;
    }
    for coord in board.get_empty_cells_coords() {
        _ = board.make_move(Move::new(coord, board.get_current_player()));
        if board.check_prev_player_for_win() {
            candidates[depth] = Some(coord);
            return;
        }
        if depth > 0 {
            if candidates[..(depth - 1)]
                .iter()
                .map(|f| Into::<usize>::into(f.is_some()))
                .sum::<usize>()
                >= 1
            {
                _ = board.undo_move();

                break;
            }
        }
        let new_depth = depth + 1;
        deeperest_recursion(board.clone(), new_depth, max_depth, candidates);

        _ = board.undo_move();
    }
}
const DEEPEREST_MAX_DEPTH: usize = 4;
fn deeperest(board: Board) -> (usize, usize) {
    let mut candidates: [Option<(usize, usize)>; DEEPEREST_MAX_DEPTH] = [None; DEEPEREST_MAX_DEPTH];
    _ = deeperest_recursion(board.clone(), 0, DEEPEREST_MAX_DEPTH, &mut candidates);
    // println!("{:?}", candidates);
    for i in candidates {
        match i {
            Some(m) => return m,
            None => {}
        }
    }
    return board.get_empty_cells_coords()[0];
}
#[cfg(test)]
mod tests {
    use crate::{board::Board, bot::deeperest, player::Player, r#move::Move};

    #[test]
    fn obvius_positions_3x3() {
        let player1 = Player::new(1, 'X');
        let player2 = Player::new_custom(2, 'O', crate::player::PlayerType::Deeper);

        let mut board = Board::new(3, 3, 3, [player1, player2].to_vec());
        _ = board.make_move(Move::new((0, 0), player1));
        _ = board.make_move(Move::new((1, 1), player2));
        _ = board.make_move(Move::new((1, 0), player1));
        _ = board.make_move(Move::new((2, 0), player2));
        _ = board.make_move(Move::new((0, 2), player1));
        let bot_move = deeperest(board.clone());
        assert_eq!(
            bot_move,
            (0, 1),
            "position \n{}\nbot_move {:?}",
            board,
            bot_move
        );

        let mut board = Board::new(3, 3, 3, [player1, player2].to_vec());
        _ = board.make_move(Move::new((0, 0), player1));
        _ = board.make_move(Move::new((1, 1), player2));
        _ = board.make_move(Move::new((1, 0), player1));
        let bot_move = deeperest(board.clone());
        assert_eq!(
            bot_move,
            (2, 0),
            "position \n{}\nbot_move {:?}",
            board,
            bot_move
        );
        let mut board = Board::new(3, 3, 3, [player1, player2].to_vec());
        _ = board.make_move(Move::new((0, 0), player1));
        _ = board.make_move(Move::new((1, 1), player2));
        _ = board.make_move(Move::new((1, 0), player1));
        _ = board.make_move(Move::new((2, 0), player2));
        _ = board.make_move(Move::new((0, 1), player1));
        let bot_move = deeperest(board.clone());
        assert_eq!(
            bot_move,
            (0, 2),
            "position \n{}\nbot_move {:?}",
            board,
            bot_move
        );
        let mut board = Board::new(3, 3, 3, [player1, player2].to_vec());
        _ = board.make_move(Move::new((0, 0), player1));
        _ = board.make_move(Move::new((0, 2), player2));
        _ = board.make_move(Move::new((1, 1), player1));
        let bot_move = deeperest(board.clone());
        assert_eq!(
            bot_move,
            (2, 2),
            "position \n{}\nbot_move {:?}",
            board,
            bot_move
        )
    }
}
