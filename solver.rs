
use tokenize::Token;
use tokenize::TokenType;
use tokenize::Operation;
use tokenize::print_tokens;

pub fn solve(tokens: Vec<Token>) -> i64 {

	if tokens.len() == 0 {
		println!("Something went horribly wrong! there are no tokens!");
		return 0;
	}

	//dbg!(&tokens);
	print_tokens(&tokens);

	// remove unary -
	let newtoks = remove_unary_minus(tokens);


	println!("unary minus removed:");
	//dbg!(newtoks);
	print_tokens(&newtoks);

	return 0;
}


fn peek<'a>(i: &usize, amount: i64, vec: &'a Vec<Token>) -> Option<&'a Token>{

	let index: i64 = *i as i64;

	if index + amount < 0 { return None; }
	if index + amount >= vec.len() as i64 { return None; }

	return Some(&vec[(index+amount) as usize]);
}

fn remove_unary_minus(mut tokens: Vec<Token>) -> Vec<Token> {

	let mut newtoks = Vec::<Token>::new();
	let mut found_unary: bool;

	loop{
		found_unary = false;

		let mut i = 0;
		loop{
			if i >= tokens.len(){
				break;
			}

			if tokens[i].id == TokenType::Operation && tokens[i].value == Operation::Sub as i64 {
				let ntok = peek(&i,1,&tokens);

				if ntok.is_none() {
					break;
				}

				let ptok = peek(&i,-1,&tokens);

				if ptok.is_none() || ptok.unwrap().id == TokenType::Operation || ntok.unwrap().id == TokenType::OpenParen {

					found_unary = true;
					match ntok.unwrap().id {
						TokenType::Number => 
							{
								tokens[i+1].value = 0-tokens[i+1].value;
								newtoks.push(tokens[i+1]); // push the negated number
								i+=2; 
							},
						TokenType::OpenParen => 
							{ 
								newtoks.push(tokens[i+1]); // push the "("
								i+=2;

								loop {
									if i >= tokens.len() || tokens[i].id == TokenType::CloseParen { break; }
									if tokens[i].id == TokenType::Number { tokens[i].value = 0-tokens[i].value; } // negate all numbers
									newtoks.push(tokens[i]); // push element
									i+=1;
								}

								newtoks.push(tokens[i]); // push the ")"
								i+=1;
							},
						_ => 
							{	
								println!("MUTLIPLE OPERATIONS AFTER EACH OTHER WITH NO NUMBER!");
								std::process::exit(1);
							},
					}


				}else {
					newtoks.push(tokens[i]);
					i += 1;
				}
				
			}else{
				newtoks.push(tokens[i]);
				i+=1;
			}

		}

		if found_unary == false {
			return newtoks;
		}
	}

}