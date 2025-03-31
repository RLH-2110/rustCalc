
use tokenize::Token;
use tokenize::TokenType;
use tokenize::Operation;
use tokenize::print_tokens;
use std::process;

pub fn solve(tokens: Vec<Token>) -> i64 {

	if tokens.len() == 0 {
		println!("Something went horribly wrong! there are no tokens!");
		return 0;
	}

	// remove unary -
	let newtoks = remove_unary_minus(tokens);

	find_double_operators(&newtoks);

	//dbg!(&newtoks);
	print_tokens(&newtoks);

	return 0;
}


fn peek<'a>(i: &usize, amount: i64, vec: &'a Vec<Token>) -> Option<&'a Token>{

	let index: i64 = *i as i64;

	if index + amount < 0 { return None; }
	if index + amount >= vec.len() as i64 { return None; }

	return Some(&vec[(index+amount) as usize]);
}

fn remove_unary_minus(tokens: Vec<Token>) -> Vec<Token> {

	let mut toks = tokens.clone();
	let mut found_unary: bool;

	loop{
		let mut newtoks = Vec::<Token>::new();
		found_unary = false;

		let mut i = 0;
		loop{
			if i >= toks.len(){
				break;
			}

			if toks[i].id == TokenType::Operation && toks[i].value == Operation::Sub as i64 {
				let ntok = peek(&i,1,&toks);

				if ntok.is_none() {
					break;
				}

				let ptok = peek(&i,-1,&toks);

				if ptok.is_none() || ptok.unwrap().id == TokenType::Operation || ntok.unwrap().id == TokenType::OpenParen {

					found_unary = true;
					match ntok.unwrap().id {
						TokenType::Number => 
							{
								toks[i+1].value = 0-toks[i+1].value;
								newtoks.push(toks[i+1]); // push the negated number
								i+=2; 
							},
						TokenType::OpenParen => 
							{ 
								newtoks.push(toks[i+1]); // push the "("
								i+=2;

								loop {
									if i >= toks.len() || toks[i].id == TokenType::CloseParen { break; }
									if toks[i].id == TokenType::Number { toks[i].value = 0-toks[i].value; } // negate all numbers
									newtoks.push(toks[i]); // push element
									i+=1;
								}

								newtoks.push(toks[i]); // push the ")"
								i+=1;
							},
						_ => 
							{	
								println!("MUTLIPLE OPERATIONS AFTER EACH OTHER WITH NO NUMBER!");
								std::process::exit(1);
							},
					}


				}else {
					newtoks.push(toks[i]);
					i += 1;
				}
				
			}else{
				newtoks.push(toks[i]);
				i+=1;
			}

		}

		toks = newtoks;
		if found_unary == false {
			return toks;
		}
	}
}

fn find_double_operators(tokens: &Vec<Token>){

	let mut had_op = false;

	for token in tokens{
		if token.id == TokenType::Operation {
			if had_op {
				println!("Double operators found!");
				process::exit(0);
			}else{
				had_op = true;
			}
		}else{
			had_op = false;
		}
	}

}