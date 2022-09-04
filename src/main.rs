#![allow(dead_code)]

mod estruturas;
mod io;

use crate::io::{tui,tui::Act};



use std::error;

fn main() {
	if let Err(e) = program_loop(){
		println!("❌ Erro na execução, descrição do erro é:\n\t{}",e);
	};
}


fn program_loop()->Result<(),Box<dyn error::Error>>{
	loop{
		let act = tui::get_action()?;
		match act {
			Act::NameSearch(name) => {
				println!("name: {}",name);
			},
			Act::VisitedPlayers(user_id) => {
				println!("user_id: {}",user_id);
			},
			Act::Top10Position(position) => {
				println!("position: {}",position);
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