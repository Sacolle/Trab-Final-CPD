use dialoguer::{FuzzySelect, theme::ColorfulTheme, Input};
use ansi_term::{Style, Color::{Black, White}};

use std::error::Error;


const OPTIONS:[&'static str;5] = ["Player Search","User Reviews","Top 10 at position","Search Player by Tags","Exit"];
pub enum Act{
	NameSearch(String),
	VisitedPlayers(usize),
	TopPosition(usize, String),
	SearchTags(Vec<String>),
	Exit
}

pub fn prog_intro(){
	println!("\n\t{}\n",Style::new().bold().paint("Trabalho Final de Classificação e Pesquisa de Dados"));
	println!("{} Pedro Henrique Boniatti Colle",Black.on(White).paint("Feito por:"));
	println!("{} 00333916", Black.on(White).paint("Matrícula:"));
	println!("{} 2022/1\n", Black.on(White).paint("Semestre :"));
}

pub fn get_action()->Result<Act,Box<dyn Error>>{
	let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
		.with_prompt("Selecione uma opção")
		.items(&OPTIONS)
		.default(0)
		.interact()?;

	//match do index do elemento da lista de opções selecionados
	match selection{
		0 => {
			let input = get_val("Insira o nome ou prefixo de busca:")?;
			return Ok(Act::NameSearch(input));
		},
		1 => {
			let user_id:usize = get_val("Insira o id do usuário de busca:")?
				.trim()
				.parse()?;

			return Ok(Act::VisitedPlayers(user_id));
		},
		2 => {
			let amount:usize = get_val("Insira o um numero N para retornar o Top N: ")?
				.trim()
				.parse()?;
			
			let postion = get_val("Insira a posição de busca:")?;
			return Ok(Act::TopPosition(amount, postion));
		},
		3 => {
			let input = get_val("Insira as tags de busca")?;
			let tags:Vec<String> = input.split(' ').map(|tag|tag.to_owned()).collect();
			return Ok(Act::SearchTags(tags));
		},
		_ => return Ok(Act::Exit)
	}
}

fn get_val(prompt: &str) -> Result<String,Box<dyn Error>>{
	Ok(Input::with_theme(&ColorfulTheme::default())
		.with_prompt(prompt)
		.interact_text()?)
}

