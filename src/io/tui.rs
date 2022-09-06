use dialoguer::{FuzzySelect, theme::ColorfulTheme, Input};
use std::error::Error;

/*
* uso da biblioteca dialoguer para extrair a informação de busca do user
* o seletor usa o fuzzy select dentre uma lista de opções
*/

const OPTIONS:[&'static str;5] = ["Player Search","User Reviews","Top 10 at position","Search Player by Tags","Exit"];
pub enum Act{
	NameSearch(String),
	VisitedPlayers(usize),
	TopPosition(usize, String),
	SearchTags(Vec<String>),
	Exit
}

pub fn get_action()->Result<Act, Box<dyn Error>>{
	let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
		.with_prompt("Selecione uma opção")
		.items(&OPTIONS)
		.default(0)
		.interact()?;

	//match do index do elemento da lista de opções selecionados
	//realiza-se o parse apropriado por item
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
			//uso de funções de alta ordem para processar as tags
			let tags:Vec<String> = input
				.split('\'')	//divide-se nos '
				.map(|tag|tag.trim().to_owned()) //remove-se espaços vazios
				.filter(|elem|!elem.is_empty()) //remove-se strings vazias
				.collect();

			return Ok(Act::SearchTags(tags));
		},
		_ => return Ok(Act::Exit)
	}
}

//helper funcion para pedir a entrada dada uma prompt
fn get_val(prompt: &str) -> Result<String,Box<dyn Error>>{
	Ok(Input::with_theme(&ColorfulTheme::default())
		.with_prompt(prompt)
		.interact_text()?)
}

