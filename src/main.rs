use std::io;
use rand::seq::SliceRandom;
use crate::player::{Player, PlayerType};

mod game;
mod board;
mod cell;
mod r#move;
mod player;

fn main() {
    let players = vec![Player::new(1, 'X'), Player::new_bot(2, 'O')];
    let game_manager = game::Game::new((4, 4), &players);
    game_loop(game_manager, &players);
}
#[derive(PartialEq, Debug)]
enum GameState {
    Running,
    Closing,
}

fn game_loop(mut game_manager: game::Game, players: &Vec<Player>) {
    let mut game_state = GameState::Running;
    let mut move_counter = 0;
    loop {
        let player = &players[move_counter % players.len()];
        move_counter += 1;
        loop {
            let input = if player.p_type == PlayerType::Terminal {
                ask_terminal()
            } else {
                ask_bot(game_manager.clone(), &player)
            };
            match input_handler(input) {
                Ok(input) => match input {
                    Command::Exit => {
                        println!("Are you shure? The game is running!");
                        if terminal_said_yes() {
                            game_state = GameState::Closing
                        }
                    }
                    Command::ChooseCell(coord) => match game_manager.make_move(coord, *player, move_counter) {
                        Ok(_) => break,
                        Err(err) => println!("{:?}", err),
                    },
                },
                Err(err) => match err {
                    InputError::Error(msg) => println!("Err: {msg}"),
                },
            };
            if game_state == GameState::Closing {
                break;
            }
        }
        if game_state == GameState::Closing {
            break;
        }

        game_manager.draw_board();
        match game_manager.game_win_draw_checker(player.id) {
            game::WinDraw::Win => {
                println!("{:?}, won!", player);
                break;
            }
            game::WinDraw::Draw => {
                println!("Draw!");
                break;
            }
            game::WinDraw::Noting => {}
        }
    }
}

fn janky_bot() -> (usize, usize) {
    let mut coord: (usize, usize) = (0, 0);
    coord.0 = [0, 1, 2, 3]
        .choose(&mut rand::thread_rng())
        .unwrap()
        .to_owned();
    coord.1 = [0, 1, 2, 3]
        .choose(&mut rand::thread_rng())
        .unwrap()
        .to_owned();
    return coord;
}
fn deeper(mut game_manager: game::Game, player: &Player) -> (usize, usize){
    let best_move: (usize, usize) = (0,0);
    return best_move;
}

fn ask_bot(game_manager: game::Game, player: &Player) -> String {
    // let moove = janky_bot();
    let moove = deeper(game_manager, player);
    return "mk ".to_string() + moove.0.to_string().as_str() + " " + moove.1.to_string().as_str();
}
fn ask_terminal() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    return input;
}
fn terminal_said_yes() -> bool {
    let mut counter: u8 = 0;
    while counter < 3 {
        match ask_terminal().trim().to_lowercase().as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => counter += 1,
        }
    }
    return false;
}
pub enum InputError {
    Error(String),
}
enum Command {
    Exit,
    ChooseCell((usize, usize)),
}
fn input_handler(input: String) -> Result<Command, InputError> {
    let command = input.trim().split(' ').collect::<Vec<_>>();
    match command[0] {
        "exit" => Ok(Command::Exit),
        "mk" => {
            let mut coord: (usize, usize) = (0, 0);

            coord.1 = command
                .get(1)
                .ok_or(InputError::Error("X Y required".to_string()))?
                .parse()
                .ok()
                .ok_or(InputError::Error(
                    "coordinate shoud be a number".to_string(),
                ))?;

            coord.0 = command
                .get(2)
                .ok_or(InputError::Error("Err: X Y required".to_string()))?
                .parse()
                .ok()
                .ok_or(InputError::Error(
                    "coordinate shoud be a number".to_string(),
                ))?;
            return Ok(Command::ChooseCell(coord));
        }
        _ => Err(InputError::Error("Command not found".to_string())),
    }
}
