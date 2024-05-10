use crate::{
    board::{Board, BoardError},
    player::{Player, PlayerType},
    r#move::Move,
    utils,
};
use rand::seq::SliceRandom;
use std::io::{stdin, stdout, Write};
use std::usize;
use utils::{LOBBY_HELP_MESSAGE, SKINS};
#[derive(Debug)]
pub enum GameResult {
    Win(Player),
    GameStopped,
    Draw,
}
enum LobbyError {
    Error(String),
}
#[derive(PartialEq)]
enum LobbyState {
    Waiting,
    Running,
    Closing,
}
pub struct Lobby {
    players: Vec<Player>,
    lobby_state: LobbyState,
    board_settings: (usize, usize),
}
impl Lobby {
    pub fn new() -> Self {
        Lobby {
            players: vec![],
            lobby_state: LobbyState::Waiting,
            board_settings: (4, 4),
        }
    }
    pub fn add_players(&mut self, mut players: Vec<Player>) {
        self.players.append(&mut players)
    }
    pub fn is_game_running(&self) -> bool {
        if self.lobby_state == LobbyState::Running {
            return true;
        }
        false
    }
    pub fn run(&mut self) {
        println!("welcome to lobby!");
        loop {
            let input: String = input(">>>");
            match lobby_input_handler(input) {
                Ok(input) => match input {
                    LobbyCommand::Exit => {
                        println!("Exiting...");
                        self.lobby_state = LobbyState::Closing;
                    }

                    LobbyCommand::Help => println!("{}", LOBBY_HELP_MESSAGE),
                    LobbyCommand::StartGame => match self.start_game() {
                        Ok(v) => println!("{:?}", v),
                        Err(v) => println!("{:?}", v),
                    },
                    LobbyCommand::ChangeBoardSize(s) => self.board_settings = (s.0, s.1),
                    LobbyCommand::AddBot => self.add_players(vec![Player::new_bot(
                        self.players.len(),
                        SKINS[self.players.len()],
                    )]),
                    LobbyCommand::RemoveBot => {
                        match self.players.iter().find(|p| p.p_type == PlayerType::Bot) {
                            Some(v) => _ = self.players.remove(v.id - 1),
                            None => println!("No bot left!"),
                        }
                    }
                },
                Err(err) => match err {
                    InputError::Error(msg) => println!("Err: {msg}"),
                },
            };
            if self.lobby_state == LobbyState::Closing {
                break;
            }
        }
    }
    fn start_game(&mut self) -> Result<GameResult, BoardError> {
        let mut game_state = GameState::Running;
        let mut board = Board::new(
            self.board_settings.0,
            self.board_settings.1,
            3,
            self.players.clone(),
        );
        println!("game started");
        print!("{board}");
        loop {
            board.set_player();
            loop {
                let input: String = if board.get_current_player().p_type == PlayerType::Terminal {
                    input(">>>")
                } else {
                    ask_bot(board.clone())
                };
                match game_input_handler(input) {
                    Ok(input) => match input {
                        GameCommand::Exit => {
                            println!("Are you shure? The game is running!");
                            if terminal_said_yes() {
                                game_state = GameState::Closing
                            }
                        }
                        GameCommand::ChooseCell(coord) => {
                            match board.make_move(Move::new(coord, board.get_current_player())) {
                                Ok(_) => break,
                                Err(err) => println!("{:?}", err),
                            }
                        }
                        GameCommand::Help => println!("{}", utils::GAME_HELP_MESSAGE),
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
                return Ok(GameResult::GameStopped);
            }

            println!("{}", board);
            if board.check_current_player_for_win() {
                println!("{:?}, won!", board.get_current_player());
                return Ok(GameResult::Win(board.get_current_player()));
            }
            if board.check_position_for_draw() {
                println!("Draw!");
                return Ok(GameResult::Draw);
            }
        }
    }
}

#[derive(PartialEq, Debug)]
enum GameState {
    Running,
    Closing,
    Waiting,
}
//plays random moves
fn janky_bot(board: Board) -> (usize, usize) {
    return board
        .get_empty_cells_coords()
        .choose(&mut rand::thread_rng())
        .unwrap()
        .to_owned();
}
fn iter_search(mut board: Board, me: Player, max_dept: usize, mut depth:usize) -> (usize, usize){
    let all_moves = board.get_empty_cells_coords();
    let mut move_to_play = all_moves[0];
    depth += 1;
    
    for coord in board.get_empty_cells_coords() {
        _ = board.make_move(Move::new(coord, board.get_current_player()));
        if board.check_current_player_for_win() && board.get_current_player() == me {
            move_to_play = coord;
        }
        else if depth < max_dept{
            move_to_play = iter_search(board.clone(), me, max_dept, depth)
        }
        _ = board.undo_move();
    }
    return move_to_play;
}
fn deeper(board: Board) -> (usize, usize) {
    let me = board.get_current_player();
    let move_to_play = iter_search(board.clone(), me, 5, 0);
    return move_to_play;
}

fn ask_bot(board: Board) -> String {
    // let moove = janky_bot();
    let moove = deeper(board);
    return "mk ".to_string() + moove.1.to_string().as_str() + " " + moove.0.to_string().as_str();
}

fn input(prompt: impl std::fmt::Display) -> String {
    let mut s = String::new();
    print!("{}", prompt);
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    s
}

fn terminal_said_yes() -> bool {
    match input("y/N: ").trim().to_lowercase().as_str() {
        "y" | "yes" => return true,
        "n" | "no" => return false,
        _ => false,
    }
}
enum InputError {
    Error(String),
}
enum LobbyCommand {
    Exit,
    Help,
    StartGame,
    ChangeBoardSize((usize, usize)),
    AddBot,
    RemoveBot,
}
enum GameCommand {
    Help,
    Exit,
    ChooseCell((usize, usize)),
}
fn lobby_input_handler(input: String) -> Result<LobbyCommand, InputError> {
    let command = input.trim().split(' ').collect::<Vec<_>>();
    match command[0] {
        "help" => Ok(LobbyCommand::Help),
        "exit" => Ok(LobbyCommand::Exit),
        "start" => Ok(LobbyCommand::StartGame),
        "chbd" => {
            let mut size: (usize, usize) = (0, 0);

            size.1 = command
                .get(1)
                .ok_or(InputError::Error("X Y required".to_string()))?
                .parse()
                .ok()
                .ok_or(InputError::Error("size shoud be a number".to_string()))?;

            size.0 = command
                .get(2)
                .ok_or(InputError::Error("X Y required".to_string()))?
                .parse()
                .ok()
                .ok_or(InputError::Error("size shoud be a number".to_string()))?;
            return Ok(LobbyCommand::ChangeBoardSize(size));
        }
        "addbt" => Ok(LobbyCommand::AddBot),
        "rmb" => Ok(LobbyCommand::RemoveBot),

        _ => Err(InputError::Error("Command not found".to_string())),
    }
}
fn game_input_handler(input: String) -> Result<GameCommand, InputError> {
    let command = input.trim().split(' ').collect::<Vec<_>>();
    match command[0] {
        "help" => Ok(GameCommand::Help),
        "exit" => Ok(GameCommand::Exit),
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
                .ok_or(InputError::Error("X Y required".to_string()))?
                .parse()
                .ok()
                .ok_or(InputError::Error(
                    "coordinate shoud be a number".to_string(),
                ))?;
            return Ok(GameCommand::ChooseCell(coord));
        }
        _ => Err(InputError::Error("Command not found".to_string())),
    }
}
