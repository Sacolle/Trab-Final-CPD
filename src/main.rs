mod estruturas;
mod io;

use crate::io::{
	tui, tui::Act,
	parser::{ parse_tags, parse_player_and_user},
	disp,
	//parser::{ User, Player}
};


use std::error;

fn main() {
	tui::prog_intro();
	if let Err(e) = program_loop(){
		println!("\n❌ Erro na execução, descrição do erro é:\n\t{}",e);
	};
}

fn program_loop()->Result<(),Box<dyn error::Error>>{
	let (player_table,
		user_table,
		_player_trie) = parse_player_and_user()?;

	//let tags_table = parse_tags()?;

	loop{
		match tui::get_action()? {
			Act::NameSearch(name) => {
				println!("name: {}",name);
			},
			Act::VisitedPlayers(user_id) =>
			{
				if let Some(user) = user_table.get(&user_id){
					disp::head_of_user_query();
					let players_id = &user.ratings;
					for (p_id, user_raiting) in players_id {
						if let Some(player) = player_table.get(p_id){
							disp::display_row_user_reviews(player, user_raiting);
						}
					}
				}else{
					println!("Usuário de id {} não encontrado",user_id);
				}
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


