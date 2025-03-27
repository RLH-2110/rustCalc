use std::env;

fn main(){
	let orig_argv: Vec<String> = env::args().collect();
    let mut argc = orig_argv.len();

	if argc == 1{
		println!("usage: {} [expression]",orig_argv[0]);
	}

	let argv = &orig_argv[1..]; argc-=1;

	let mut i = 0;
	loop{
		if i >= argc{
			break;
		}
		println!("{}",argv[i]); i+=1;
	}

}	

	

