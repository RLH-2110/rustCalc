use std::env;

mod tokenize;
mod solver;
mod functions;

fn main(){
  let argv: Vec<String> = env::args().collect();
  if argv.len() == 1{
    println!("usage: {} [expression]",argv[0]);
    std::process::exit(-1);
  }

  let args: String = argv[1..].join("");
  let expression = match tokenize::parse(args){
    Ok(e) => e,
    Err(_) => {return;},
  };
  let result = match solver::solve(expression){ 
    Ok(n) => n,    
    Err(e) => {std::process::exit(e);},
  };

  println!("{result}");
}  

