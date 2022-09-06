use ansi_term::{Style,Color::{Black,White}};

use crate::io::parser::{Player};
use core::time;

/*
* módulo com funções para realizar o display estilizado ao terminal das informações
* usa a biblioteca ansi_term para adicionar estilo ao display.
*/

pub fn prog_intro(){
	println!("\n\t{}\n",Style::new().bold().paint("Trabalho Final de Classificação e Pesquisa de Dados"));
	println!("{} Pedro Henrique Boniatti Colle",Black.on(White).paint("Feito por:"));
	println!("{} 00333916", Black.on(White).paint("Matrícula:"));
	println!("{} 2022/1\n", Black.on(White).paint("Semestre :"));
}

pub fn head_display_time(){
	println!("Processando os arquivos de entrada...");
}

pub fn display_time_elapsed(time:time::Duration){
	println!("\nTempo de execução {}\n",
		Style::new().bold().underline().paint(format!("{:.3?}",time))
	);
}

pub fn head_row_player(){
	println!("{}",Style::new().bold().underline().paint(
		format!("\n{:>9}|{:>40}|{:>17}|{:>7}|{:>6}",
			"sofifa_id",
			"name",
			"player_positions",
			"rating",
			"count"
		)
	));
}

pub fn display_row_player(p : &Player){
	println!("{:>9}|{:>40}|{:>17}|{:>7.2}|{:>6}",
		p.id,
		p.name,
		p.positions,
		p.rating,
		p.count);
}

pub fn head_row_user_review(){
	println!("{}",Style::new().bold().underline().paint(
		format!("\n{:>9}|{:>40}|{:>14}|{:>6}|{:>7}",
			"sofifa_id",
			"name",
			"global_rating",
			"count",
			"rating"
		)
	));
}

pub fn display_row_user_reviews(p : &Player, raiting: &f64){
	println!("{:>9}|{:>40}|{:>14.8}|{:>6}|{:>7.2}",
		p.id,
		p.name,
		p.rating,
		p.count,
		raiting);
}