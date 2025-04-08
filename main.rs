use std::env;

mod tokenize;
mod solver;
mod functions;

/*
  return codes:
   0 - no error
   1 - no arguments provided
   2 - invalid token
   3 - overflow
   4 - number input error
   5 - parentesis error
   6 - No tokens found
   7 - Too many operators
   8 - Leading operator
   9 - Trailing operator
  10 - Division by 0
  11 - Calculation took too long, may be an infinite loop

*/

fn main(){
  let argv: Vec<String> = env::args().collect();
  if argv.len() == 1{
    println!("usage: {} [expression]",argv[0]);
    std::process::exit(-1);
  }

  let args: String = argv[1..].join("");
  let expression = match tokenize::parse(args){
    Ok(v) => v,
    Err(e) => {std::process::exit(e)},
  };
  let result = match solver::solve(expression){ 
    Ok(n) => n,    
    Err(e) => {std::process::exit(e);},
  };

  println!("{result}");
}  

