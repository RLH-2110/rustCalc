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
	match a.checked_add(*b) {
		Some(r) => {return r;},
		None => {overflow();0},
	}
}

fn sub(a: &i64, b: &i64) -> i64 {
	match a.checked_sub(*b) {
		Some(r) => {return r;},
		None => {overflow();0},
	}
}

fn mul(a: &i64, b: &i64) -> i64 {
	match a.checked_mul(*b) {
		Some(r) => {return r;},
		None => {overflow();0},
	}
}

fn div(a: &i64, b: &i64) -> i64 {
	if *b == 0{
		println!("DIVISION BY ZERO!");
		std::process::exit(1);
	}
	match a.checked_div(*b) {
		Some(r) => {return r;},
		None => {overflow();0},
	}
}

fn overflow(){
	println!("OVERFLOW!");
	std::process::exit(1);
}