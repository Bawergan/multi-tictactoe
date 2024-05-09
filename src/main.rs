use crate::player::Player;
use board::Board;
use lobby::Lobby;

mod board;
mod cell;
mod lobby;
mod r#move;
mod player;
mod utils;

fn main() {
    println!("type help to list all commands");
    let players = vec![Player::new(1, 'X'), Player::new_bot(2, 'O')];
    let _board = Board::new(4, 4);

    let mut lobby: Lobby = Lobby::new();
    lobby.add_players(players);

    lobby.run()
}
