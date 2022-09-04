#![allow(dead_code)]

mod estruturas;
mod io;

use crate::io::{
	tui, tui::Act,
	parser::{parse_tags,parse_player_and_user}
};

use std::error;

fn main() {
	tui::prog_intro();
	if let Err(e) = program_loop(){
		println!("\n❌ Erro na execução, descrição do erro é:\n\t{}",e);
	};
}

fn program_loop()->Result<(),Box<dyn error::Error>>{
	let (player_table,user_table) = parse_player_and_user()?;
	let tags_table = parse_tags()?;


	loop{
		match tui::get_action()? {
			Act::NameSearch(name) => {
				println!("name: {}",name);
			},
			Act::VisitedPlayers(user_id) => {
				println!("user_id: {}",user_id);
			},
			Act::TopPosition(amount ,position) => {
				println!("Top{} position: {}",amount,position);
			},
			Act::SearchTags(tags) => {
				println!("tags: {:?}",tags);
			},
			Act::Exit => {
				return Ok(());
			},
		}
	}
}