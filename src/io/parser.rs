use crate::estruturas::hash_table::{HashTable,hash_usize,hash_string};

use serde::Deserialize;
use csv::Reader;

use std::error::Error;

#[derive(Deserialize)]
struct SerdePlayer{
	sofifa_id:usize,
	name: String,
	player_position: String
}

#[derive(Debug)]
pub struct Player{
	id: usize,
	name: String,
	positions: String,
	rating: f64,
	count: i32
}

#[derive(Deserialize)]
struct SerdeUser{
	user_id:usize,
	sofifa_id: usize,
	raiting: f64
}

#[derive(Debug)]
pub struct User{
	id:usize,
	ratings: Vec<(usize,f64)>
}

#[derive(Deserialize)]
struct SerdeTag{
	user_id: usize,
	sofifa_id: usize,
	tag: String
}

impl Player{
	fn new(player:SerdePlayer)-> Self{
		Player {
			id: player.sofifa_id,
			name: player.name,
			positions: player.player_position,
			rating: 0.0,
			count: 0
		}
	}
	fn add(&mut self,raiting:f64){
		self.rating += raiting;
		self.count += 1;
	}
}

impl User{
	fn new(raiting: SerdeUser)->Self{
		User{
			id: raiting.user_id,
			ratings: vec![(raiting.sofifa_id,raiting.raiting)]
		}
	}
	fn add(&mut self, raiting: SerdeUser){
		self.ratings.push((raiting.sofifa_id,raiting.raiting));
	} 
}


pub fn parse_player_and_user()->Result<(HashTable<usize, Player>, HashTable<usize, User>),Box<dyn Error>>{
	let mut player_table: HashTable<usize, Player> = HashTable::new(25000,hash_usize);
	let mut user_table: HashTable<usize, User> = HashTable::new(25000,hash_usize);

	//lê o csv dos players
	let mut rdr = Reader::from_path("data/players.csv")?;
	let players = rdr.deserialize::<SerdePlayer>();

	for _player in players {
		let player = Player::new(_player?);
		player_table.insert(player.id, player);
	}

	//lê o csv dos users
	let mut rdr = Reader::from_path("data/raitings.csv")?;
	let raitings = rdr.deserialize::<SerdeUser>();

	for _raiting in raitings {
		let raiting = _raiting?;

		//adiciona as avaliações ao estruct de player
		if let Some(player) = player_table.get_mut(&raiting.sofifa_id){
			player.add(raiting.raiting);
		}

		//adicionar as avaliações do user a tabela
		if let Some(user) = user_table.get_mut(&raiting.user_id){
			user.add(raiting);
		}else{
			let user = User::new(raiting);
			user_table.insert(user.id, user);
		}
	}
	Ok((player_table,user_table))
}


pub fn parse_tags() -> Result<HashTable<String,Vec<usize>>,Box<dyn Error>>{
	let mut tags_table:HashTable<String, Vec<usize>> = HashTable::new(100,hash_string);

	let mut rdr = Reader::from_path("data/tags.csv")?;
	let tags = rdr.deserialize::<SerdeTag>();

	for _tag_row in tags {
		let tag_row = _tag_row?;

		//adicionar as avaliações do user a tabela
		if let Some(tag) = tags_table.get_mut(&tag_row.tag){
			tag.push(tag_row.sofifa_id);
		}else{
			tags_table.insert(tag_row.tag, vec![tag_row.sofifa_id]);
		}
	}
	Ok(tags_table)
}