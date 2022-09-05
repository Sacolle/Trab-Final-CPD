mod estruturas;
mod io;

use crate::io::{
	tui, tui::Act,
	parser::{ parse_tags, parse_player_and_user},
	disp,
	//parser::{ User, Player}
};


use std::{error,time};

fn main() {
	tui::prog_intro();
	if let Err(e) = program_loop(){
		println!("\n❌ Erro na execução, descrição do erro é:\n\t{}",e);
	};
}

fn program_loop()->Result<(),Box<dyn error::Error>>{
	let timer = time::Instant::now();
	disp::head_display_time();

	let (player_table,
		user_table,
		player_trie) = parse_player_and_user()?;

	let _tags_table = parse_tags()?;

	disp::display_time_elapsed(timer.elapsed());

	crate::estruturas::hash_table::utils::entries(&player_table);

	loop{
		match tui::get_action()? {
			Act::NameSearch(player_name) => {
				disp::head_row_player();
				for id in player_trie.get_prefix(&player_name){
					if let Some(player) = player_table.get(&id){
						disp::display_row_player(player);
					}
				}
			},
			Act::VisitedPlayers(user_id) =>
			{
				if let Some(user) = user_table.get(&user_id){
					disp::head_row_user_review();
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


