use crate::estruturas::{
	hash_table::{HashTable,
		utils::{hash_usize,hash_string,divide_raiting}
	},
	trie_ternary::Trie
};



use serde::Deserialize;
use csv::Reader;

use std::error::Error;

#[derive(Deserialize)]
struct SerdePlayer{
	sofifa_id:usize,
	name: String,
	player_positions: String
}

#[derive(Debug,PartialEq)]
pub struct Player{
	pub id: usize,
	pub name: String,
	pub positions: String,
	pub rating: f64,
	pub count: i32
}

#[derive(Deserialize)]
struct SerdeUser{
	user_id:usize,
	sofifa_id: usize,
	rating: f64
}

#[derive(Debug)]
pub struct User{
	pub id:usize,
	pub ratings: Vec<(usize,f64)>
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct SerdeTag{
	user_id: usize,
	sofifa_id: usize,
	tag: String
}

//posíveis posições: {"LW", "LM", "CM", "CDM", "LB", "GK", "RB", "RM", "RWB", "CB", "CAM", "ST", "LWB", "CF", "RW"}
impl Player{
	fn new(player:SerdePlayer)-> Self{
		Player {
			id: player.sofifa_id,
			name: player.name,
			positions: player.player_positions,
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
			ratings: vec![(raiting.sofifa_id,raiting.rating)]
		}
	}
	fn add(&mut self, raiting: SerdeUser){
		self.ratings.push((raiting.sofifa_id,raiting.rating));
	} 
}

pub fn parse_player_and_user()->Result<(
		HashTable<usize, Player>,
		HashTable<usize, User>,
		HashTable<String, Vec<usize>>,
		Trie<usize>
	),Box<dyn Error>>
	{

	let mut player_table: HashTable<usize, Player> = HashTable::new(25000,hash_usize);
	let mut user_table: HashTable<usize, User> = HashTable::new(25000,hash_usize);
	let mut pos_table: HashTable<String, Vec<usize>> = HashTable::new(20,hash_string);

	let mut trie: Trie<usize> = Trie::init();

	//lê o csv dos players
	let mut rdr = Reader::from_path("data/players.csv")?;
	let players = rdr.deserialize::<SerdePlayer>();

	for p in players {
		let player = Player::new(p?);

		//inserção na hashtable das posições
		player.positions
			.split(',')
			.map(|v|v.trim().to_owned())
			.for_each(|p|{
				if let Some(pos) = pos_table.get_mut(&p){
					pos.push(player.id);
				}else{
					pos_table.insert(p, vec![player.id]);
				}
			});
		
		//inserção do player na trie
		trie.insert_str(&player.name, &player.id);

		player_table.insert(player.id, player);
	}

	//lê o csv dos users
	let mut rdr = Reader::from_path("data/rating.csv")?;
	let raitings = rdr.deserialize::<SerdeUser>();

	for _raiting in raitings {
		let raiting = _raiting?;

		//adiciona as avaliações ao estruct de player
		if let Some(player) = player_table.get_mut(&raiting.sofifa_id){
			player.add(raiting.rating);
		}

		//adicionar as avaliações do user a tabela
		if let Some(user) = user_table.get_mut(&raiting.user_id){
			user.add(raiting);
		}else{
			let user = User::new(raiting);
			user_table.insert(user.id, user);
		}
	}

	divide_raiting(&mut player_table);

	Ok((player_table, user_table, pos_table, trie))
}


pub fn parse_tags() -> Result<HashTable<String,HashTable<usize,usize>>,Box<dyn Error>>{
	let mut tags_table:HashTable<String,HashTable<usize,usize>> = HashTable::new(100,hash_string);

	let mut rdr = Reader::from_path("data/tags.csv")?;
	let tags = rdr.deserialize::<SerdeTag>();

	for _tag_row in tags {
		let tag_row = _tag_row?;

		if let Some(tag) = tags_table.get_mut(&tag_row.tag){
			if let None = tag.get(&tag_row.sofifa_id){
				tag.insert(tag_row.sofifa_id,tag_row.sofifa_id);
			}
		}else{
			tags_table.insert(tag_row.tag, HashTable::new(1000, hash_usize));
		}
	}
	Ok(tags_table)
}

