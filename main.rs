use std::env;

mod tokenize;


fn main(){
	let argv: Vec<String> = env::args().collect();
	if argv.len() == 1{
		println!("usage: {} [expression]",argv[0]);
		return;
	}

	let args: String = argv[1..].join("");
	let expression = tokenize::parse(args);
	dbg!(expression);
}	






