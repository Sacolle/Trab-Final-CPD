use dialoguer::{FuzzySelect, theme::ColorfulTheme, Input};

use std::error::Error;

pub enum Act{
	NameSearch(String),
	VisitedPlayers(usize),
	Top10Position(String),
	SearchTags(Vec<String>),
	Exit
}
	/*
		.author("Pedro Colle, pedro.h.b.colle@gmail.com")
		.version("0.0.1")
		.about("Selecione uma opção de query")
	*/



const OPTIONS:[&'static str;5] = ["Player Search","User Reviews","Top 10 at position","Search Player by Tags","Exit"];

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
			let input = get_val("Insira o id do usuário de busca:")?;
			let user_id:usize = input.trim().parse()?;
			return Ok(Act::VisitedPlayers(user_id));
		},
		2 => {
			let input = get_val("Insira a posição de busca:")?;
			return Ok(Act::Top10Position(input));
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