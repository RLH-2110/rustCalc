
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum TokenType{
	Number,
	Operation,
	OpenBrak,
	CloseBrak,

	None, /* used when saving what the previous token is, and there is no previous token*/
	Invalid,
}

#[repr(u32)]
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Operation{
	Add,
	Sub,
	Mul,
	Div,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Token {
	id: TokenType,
	value: u32,
	prio: u8,
}



pub fn parse(text: String) -> Result<Vec<Token>,u8>{

	let mut expression: Vec<Token> = Vec::new();
	let mut input: String = String::new();
	let mut id = TokenType::None;
	let mut invalid_token = false; /*for handleing newlines for invalid token error prints*/
	let mut braket_count = 0;

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
		if c == '('{
			braket_count += 1;
			if id != TokenType::OpenBrak  {
				if id != TokenType::None{
					add_token(&mut expression,&id,&mut input);
				}

				input.push(c);
				id = TokenType::OpenBrak;
				continue;
			}
			add_token(&mut expression,&id,&mut input);
			input.push(c);
			continue;
		
		}
		if c == ')'{
			braket_count -= 1;
			if id != TokenType::CloseBrak {
				if id != TokenType::None{
					add_token(&mut expression,&id,&mut input);
				}

				input.push(c);
				id = TokenType::CloseBrak;
				continue;
			}
			add_token(&mut expression,&id,&mut input);
			input.push(c);
			continue;
		
		}


		// if other
		{
			
			if id != TokenType::Invalid{
				if invalid_token { println!(""); } /*new invalid token error message*/
				print!("Unknown Token: ");

				if id != TokenType::None{
					add_token(&mut expression,&id,&mut input);
				}

				print!("{c}");
				input.push(c);
				id = TokenType::Invalid;

				invalid_token = true;
				continue;
			}
			input.push(c);
			print!("{c}");
			continue;
		}

	}
	add_token(&mut expression,&id,&mut input);
	if invalid_token { println!(""); return Err(0); }

	if braket_count != 0{
		if braket_count > 0{
			println!("there are {braket_count} unclosed brakets!");
		}else{
			println!("there are {} unopnened brakets!",0-braket_count);
		}
		return Err(1);
	}

	return Ok(expression);
}



pub fn add_token(expression: &mut Vec<Token>, id: &TokenType, input: &mut String){


	match *id{
		TokenType::Operation => {
			
			let (val,pri) = match input.as_str(){
				"+" => (Operation::Add as u32,0),
				"-" => (Operation::Sub as u32,0),
				"*" => (Operation::Mul as u32,1),
				"/" => (Operation::Div as u32,1),
				_   => (Operation::Add as u32,0),
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
