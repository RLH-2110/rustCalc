
use tokenize::Token;
use tokenize::TokenType;
//use tokenize::Operation;

pub fn solve(tokens: Vec<Token>) -> u64 {

	dbg!(&tokens);

	let mut newtoks = Vec::<Token>::new();

	//let mut i = 0;


	// step 1: remove brakets


	let mut brak_level = 0;
	for mut token in tokens{
		token.prio += brak_level;

		match token.id {
			TokenType::OpenBrak => { brak_level+=5; },
			TokenType::CloseBrak => { brak_level-=5; },
			_ => { newtoks.push(token); }
		}
	}

	let toks = newtoks;
	newtoks = Vec::<Token>::new();

	println!("brakets removed:");
	dbg!(toks);

	/*loop{


		if newtoks.len() <= 1 { break; }
	}

	if (newtoks.len() < 1){
		println!("Critical error! there was no result!");
	}
	return newtoks[0];*/
	return 0;
}


/*fn peek(i: &i64, amount: i64, vec: &Vec<Token>) -> Option<Token>{
	if *i + amount < 0 { return None; }
	if *i + amount >= vec.len().try_into().unwrap() { return None; }

	return Some(vec[*i+amount]);
}*/