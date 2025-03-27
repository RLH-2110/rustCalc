use std::env;

mod tokenize;


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


	let expression = tokenize::parse(args);
	dbg!(expression);
}	






