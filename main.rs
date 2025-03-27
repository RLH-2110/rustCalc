use std::env;

#[derive(PartialEq, Eq, Debug)]
enum TokenType{
	Number,
	Operation,
}

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
	expression.push(Token {id: TokenType::Number, value: 15});

	if expression[0].id == TokenType::Number{
		println!("Number {}",expression[0].value);
	}
	else {
		if expression[0].id == TokenType::Operation {
			println!("Operation {}",expression[0].value);
		}else{
			unimplemented!();
		}
	}
}	

	

