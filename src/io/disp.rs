//use ansi_term::{Style, Color::{Black, White}};

use crate::io::parser::{User,Player};

pub fn head_of_user_query(){
	println!("|{:>10}|{:>30}|{:>14}|{:>6}|{:>7.2}",
		"sofifa_id",
		"name",
		"global_rating",
		"count",
		"rating");
}


pub fn display_row_user_reviews(p : &Player, raiting: &f64){
	println!("|{:>10}|{:>30}|{:>14.8}|{:>6}|{:>7.2}",
		p.id,
		p.name,
		p.rating,
		p.count,
		raiting);
}