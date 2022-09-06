mod estruturas;
mod io;
mod quicksort;

use crate::io::{
	tui, disp, tui::Act,
	parser::{ parse_tags, parse_player_and_user},
};

use std::{error, time};

fn main() {
	disp::prog_intro();
	if let Err(e) = program_loop(){
		println!("\n❌ Erro na execução, descrição do erro é:\n\t{}",e);
	};
}

fn program_loop()->Result<(),Box<dyn error::Error>>{
	let timer = time::Instant::now();
	disp::head_display_time();

	//pega os valores inicializados
	let (player_table,
		user_table,
		pos_table,
		player_trie) = parse_player_and_user()?;

	let tags_table = parse_tags()?;

	disp::display_time_elapsed(timer.elapsed());

	//program loop
	'outer: loop{
		//realiza a ação selecionada
		match tui::get_action()? {
			//retorna os players com o nome/prefixo inserido
			Act::NameSearch(player_name) => {
				let timer = time::Instant::now();
				disp::head_row_player();
				//obtem o id dos players atavez da trie
				for id in player_trie.get_prefix(&player_name){
					if let Some(player) = player_table.get(&id){
						disp::display_row_player(player);
					}
				}
				disp::display_time_elapsed(timer.elapsed());
			},
			//retorna as avaliações de um usuário
			Act::VisitedPlayers(user_id) =>{
				let timer = time::Instant::now();
				//obtém as avaliações de um usuário
				if let Some(user) = user_table.get(&user_id){
					disp::head_row_user_review();
					let players_id = &user.ratings;
					//dado um id de user e avaliação, realiza o display
					for (p_id, user_raiting) in players_id {
						if let Some(player) = player_table.get(p_id){
							disp::display_row_user_reviews(player, user_raiting);
						}
					}
				}else{
					println!("Usuário de id {} não encontrado",user_id);
					continue 'outer;
				}
				disp::display_time_elapsed(timer.elapsed());
			},
			Act::TopPosition(amount ,position) => {
				let timer = time::Instant::now();
				//carrega o vetor dos players que tem a posição
				if let Some(ids) = pos_table.get(&position){
					disp::head_row_player();
					//obtem todos os players através do seu id
					let mut players = Vec::new();
					for id in ids{
						if let Some(player) = player_table.get(id){
							players.push(player);
						} 
					}
					//usa o quicksort para ordenar os players da posição
					crate::quicksort::quicksort(&mut players);
					//realiza o display de amount primeiros players
					for player in players.into_iter().take(amount){
						disp::display_row_player(player);
					}
				}else{
					println!("Posição não reconhecida: {}",position);
					continue 'outer;
				}
				disp::display_time_elapsed(timer.elapsed());
			},
			Act::SearchTags(tags) => {
				if tags.is_empty(){
					println!("Insira uma tag:");
					continue;
				}
				let timer = time::Instant::now();
				disp::head_row_player();

				//obtém todas as tables das tags inseridas
				let mut tables = Vec::new();
				for tag in tags.iter(){
					if let Some(ids) = tags_table.get(tag){
						tables.push(ids);
					}else{
						println!("Tag: {} é inválida",tag);
						continue 'outer;
					}
				}
				let first_table = tables.get(0).unwrap().all();

				for id in first_table{
					//se o id na primeira tabela de tag não existe em pelo menos uma das tabelas, pula-se esse id
					let mut inserct = true;
					for t in tables.iter().skip(1){
						if t.get(id).is_none(){
							inserct = false;
							break;
						}		
					}
					if inserct{
						if let Some(p) = player_table.get(id){
							disp::display_row_player(p);
						}
					}
				}
				disp::display_time_elapsed(timer.elapsed());
			},
			Act::Exit => {
				return Ok(());
			},
		}
	}
}


