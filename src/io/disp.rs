//use ansi_term::Color::{ White, RGB};
//White.bold().on(RGB(250, 148, 5)).paint(

use crate::io::parser::{Player};
use core::time;


pub fn head_display_time(){
	println!("Processando os arquivos de entrada...");
}

pub fn display_time_elapsed(time:time::Duration){
	println!("Tempo de execução {:.3?}",time);
}

pub fn head_row_player(){
	println!("\n{sep}{:>9}{sep}{:>40}{sep}{:>17}{sep}{:>7.2}{sep}{:>6}",
		"sofifa_id",
		"name",
		"player_positions",
		"rating",
		"count",
		sep = "|"
	);
}

pub fn display_row_player(p : &Player){
	println!("|{:>9}|{:>40}|{:>17}|{:>7.2}|{:>6}",
		p.id,
		p.name,
		p.positions,
		p.rating,
		p.count);
}

pub fn head_row_user_review(){
	println!("\n{sep}{:>9}{sep}{:>40}{sep}{:>14}{sep}{:>6}{sep}{:>7}",
		"sofifa_id",
		"name",
		"global_rating",
		"count",
		"rating",
		sep = "|"
	);
}

pub fn display_row_user_reviews(p : &Player, raiting: &f64){
	println!("|{:>9}|{:>40}|{:>14.8}|{:>6}|{:>7.2}",
		p.id,
		p.name,
		p.rating,
		p.count,
		raiting);
}