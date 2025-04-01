
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum TokenType{
	Number,
	Operation,
	OpenParen,
	CloseParen,

	None, /* used when saving what the previous token is, and there is no previous token*/
	Invalid,
}

#[repr(i64)]
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Operation{
	Add,
	Sub,
	Mul,
	Div,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Token {
	pub id: TokenType,
	pub value: i64,
	pub prio: u8,
}



pub fn parse(text: String) -> Result<Vec<Token>,u8>{

	let mut expression: Vec<Token> = Vec::new();
	let mut input: String = String::with_capacity(8);
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
			if id != TokenType::OpenParen  {
				if id != TokenType::None{
					add_token(&mut expression,&id,&mut input);
				}

				input.push(c);
				id = TokenType::OpenParen;
				continue;
			}
			add_token(&mut expression,&id,&mut input);
			input.push(c);
			continue;
		
		}
		if c == ')'{
			braket_count -= 1;

			if braket_count < 0 { // user entered more ) than possible
				println!("there are unopnened brakets!");
				return Err(1);
			}

			if id != TokenType::CloseParen {
				if id != TokenType::None{
					add_token(&mut expression,&id,&mut input);
				}

				input.push(c);
				id = TokenType::CloseParen;
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
				"+" => (Operation::Add as i64,0),
				"-" => (Operation::Sub as i64,0),
				"*" => (Operation::Mul as i64,1),
				"/" => (Operation::Div as i64,1),
				_   => (Operation::Add as i64,0),
			};

			expression.push(Token {id: *id, value: val, prio: pri});
		},

		TokenType::Number => {
			expression.push(Token {id: *id, value: input.parse::<u32>().expect("Token of Type number should contain a value") as i64, prio: 0});
		},

		_ => {
			expression.push(Token {id: *id, value: 0, prio: 0});
		}
	}


	input.clear();
}




pub fn token_to_string(token: &Token) -> String {
	match token.id {

                		
		TokenType::Operation => {
	                if token.value > Operation::Div as i64 {
                                unimplemented!();
                        }

			match unsafe { std::mem::transmute(token.value) } {
				Operation::Add => {return "+".to_string(); },  
				Operation::Sub => {return "-".to_string(); },  
				Operation::Mul => {return "*".to_string(); },  
				Operation::Div => {return "/".to_string(); }, 
			}
		},
		

		TokenType::OpenParen  => {return "(".to_string(); },
		TokenType::CloseParen => {return ")".to_string(); },
		
		TokenType::Number => {return token.value.to_string(); },

		_ => {	unimplemented!();	}
	}
}

pub fn print_tokens(tokens: &Vec<Token>){
	for token in tokens {
		print!("{}",token_to_string(&token));
	}
	print!("\n");
}
