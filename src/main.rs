use crate::{
    player::{Player, PlayerType},
    r#move::Move,
};
use board::Board;
use rand::seq::SliceRandom;
use std::io::{stdin, stdout, Write};
use lobby::Lobby;
mod board;
mod cell;
mod r#move;
mod player;
mod utils;
mod lobby;

fn main() {
    println!("type help to list all commands");
    let players = vec![Player::new(1, 'X'), Player::new_bot(2, 'O')];
    let board = Board::new(4, 4);

    let mut lobby: Lobby = Lobby::new();
    lobby.add_players(players);

    lobby.run()
}
