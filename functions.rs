use tokenize::Token;
use tokenize::TokenType;
use tokenize::Operation;

/* !! ALWAYS UPDATE THE LOOKUP TABLE WHEN THE OPERATION ENUM IN TOKENIZE.RS IS UPDATED! !! */
type MathOp = fn(&i64, &i64) -> i64;
const OP_LOOKUP: [MathOp;4] = [add,sub,mul,div];

pub fn calculate(a: &i64, b: &i64,op: &Operation) -> Token {
	let result: i64;
	unsafe {result = OP_LOOKUP[std::mem::transmute::<Operation,usize>(*op)](a,b);}

	return Token {id: TokenType::Number, value: result, prio: 0};
}

fn add(a: &i64, b: &i64) -> i64 {
	let r = a+b;
	return r;
}

fn sub(a: &i64, b: &i64) -> i64 {
	let r = a-b;
	return r;
}

fn mul(a: &i64, b: &i64) -> i64 {
	let r = a*b;
	return r;
}

fn div(a: &i64, b: &i64) -> i64 {
	if *b == 0{
		println!("DIVISION BY ZERO!");
		std::process::exit(1);
	}
	let r = a/b;
	return r;
}

