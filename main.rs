use std::env;

mod tokenize;
mod solver;

fn main(){
	let argv: Vec<String> = env::args().collect();
	if argv.len() == 1{
		println!("usage: {} [expression]",argv[0]);
		return;
	}

	let args: String = argv[1..].join("");
	let expression = match tokenize::parse(args){
		Ok(e) => e,
		Err(_) => {return;},
	};
	solver::solve(expression);
}	






