use std::env;

#[derive(PartialEq, Eq, Debug)]
enum TokenType{
	Number,
	Operation,


	None, /* used when saving what the previous token is, and there is no previous token*/
	Invalid,
}

#[derive(PartialEq, Eq, Debug)]
struct Token {
	id: TokenType,
	value: u32,
}

fn main(){
	let argv: Vec<String> = env::args().collect();
	let mut args: String = "".to_string();

	if argv.len() == 1{
		println!("usage: {} [expression]",argv[0]);
	}
	
	for arg in &argv[1..]{
		println!("{}",arg);
		args.push_str(arg);
	}
	println!("{}",args);



	let mut expression: Vec<Token> = Vec::new();
	/*expression.push(Token {id: TokenType::Number, value: 15});*/

	let mut input: String = "".to_string();
	let mut id = TokenType::None;

	for (c in args.chars()){
		if c.is_digit(10){

			if (id == TokenType::None){
				id.push(c);
				id = TokenType::Number;
				continue;
			}
			if (id != TokenType::Number){
				add_token(expression,id,input);

				id.push(c);
				id = TokenType::Number;
				continue;
			}

			id.push(c);
			continue;
		}

		if (c.contains("+-*/")){


			if (id == TokenType::None){
				id.push(c);
				id = TokenType::Operation;
				continue;
			}
			if (id != TokenType::Number){
				add_token(expression,id,input);

				id.push(c);
				id = TokenType::Operation;
				continue;
			}
			add_token(expression,id,input);
			id.push(c);
			continue;
		
		}

		if (id == TokenType::None){
				id.push(c);
				id = TokenType::Invalid;
				continue;
			}
			if (id != TokenType::Invalid){
				add_token(expression,id,input);

				id.push(c);
				id = TokenType::Invalid;
				continue;
			}
			add_token(expression,id,input);
			id.push(c);
			continue;

	}


	dbg!(expression);
}	

fn add_token(expression: mut &Vec<Token>, id: TokenType, input: mut &String){

	if (input == TokenType::Operation){
		let val = match(input){
			"+" => 0,
			"-" => 1,
			"*" => 2,
			"/" => 3,
			_ => 0,
		}
		expression.push(Token {id: id, value: val});
	}
	if (input == TokenType::Number){
		expression.push(Token {id: id, value: input.parse()});
	}
	if (input == TokenType::Invalid){
		expression.push(Token {id: id value: 0});
	}


	input = "".to_string();
}

