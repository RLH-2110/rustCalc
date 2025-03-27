
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum TokenType{
	Number,
	Operation,
	OpenBrak,
	CloseBrak,

	None, /* used when saving what the previous token is, and there is no previous token*/
	Invalid,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Token {
	id: TokenType,
	value: u32,
	prio: u8,
}



pub fn parse(text: String) -> Vec<Token>{

	let mut expression: Vec<Token> = Vec::new();
	let mut input: String = "".to_string();
	let mut id = TokenType::None;

	for c in text.chars(){

		// if number
		if c.is_digit(10){

			if id != TokenType::Number{
				if id != TokenType::None{
					add_token(&mut expression,&id,&mut input);
				}
				
				input.push(c);
				id = TokenType::Number;
				continue;
			}
			input.push(c);
			continue;
		}


		// if op
		if "+-*/".contains(c){

			if id != TokenType::Operation{
				if id != TokenType::None{
					add_token(&mut expression,&id,&mut input);
				}

				input.push(c);
				id = TokenType::Operation;
				continue;
			}
			add_token(&mut expression,&id,&mut input);
			input.push(c);
			continue;
		
		}


		// if braket
		if "()".contains(c){

			if id != TokenType::OpenBrak && id != TokenType::CloseBrak{
				if id != TokenType::None{
					add_token(&mut expression,&id,&mut input);
				}

				input.push(c);
				id = if c == '(' { TokenType::OpenBrak }else{ TokenType::CloseBrak };
				continue;
			}
			add_token(&mut expression,&id,&mut input);
			input.push(c);
			continue;
		
		}


		// if other
		{
			if id != TokenType::Invalid{
				if id != TokenType::None{
					add_token(&mut expression,&id,&mut input);
				}

				input.push(c);
				id = TokenType::Invalid;
				continue;
			}
			add_token(&mut expression,&id,&mut input);
			input.push(c);
			continue;
		}

	}
	add_token(&mut expression,&id,&mut input);

	return expression;
}



pub fn add_token(expression: &mut Vec<Token>, id: &TokenType, input: &mut String){


	match *id{
		TokenType::Operation => {
			
			let (val,pri) = match input.as_str(){
				"+" => (0,0),
				"-" => (1,0),
				"*" => (2,1),
				"/" => (3,1),
				_ => (0,0)
			};

			expression.push(Token {id: *id, value: val, prio: pri});
		},

		TokenType::Number => {
			expression.push(Token {id: *id, value: input.parse::<u32>().expect("Token of Type number should contain a value"), prio: 0});
		},

		_ => {
			expression.push(Token {id: *id, value: 0, prio: 0});
		}
	}


	input.clear();
}
