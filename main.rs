use std::env;

mod tokenize;
mod solver;
mod functions;

/*
  return codes:
   0 - no error
   1 - no tokens provided
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
  12 - invalid unary operator

*/

pub const EXIT_SUCCESS: i32 = 0;
pub const EXIT_UNUSED: i32 = 1;
pub const EXIT_INVAL_TOK: i32 = 2;
pub const EXIT_MATH_OVERFLOW: i32 = 3;
pub const EXIT_INVAL_NUM: i32 = 4;
pub const EXIT_PARENTESIS_ERR: i32 = 5;
pub const EXIT_NO_TOKS: i32 = 6;
pub const EXIT_DOUBLE_OPS: i32 = 7;
pub const EXIT_LEADING_OP: i32 = 8;
pub const EXIT_TRAILING_OP: i32 = 9;
pub const EXIT_DIV_BY_ZERO: i32 = 10;
pub const EXIT_INFINITE_LOOP: i32 = 11;
pub const EXIT_INVAL_UNARY: i32 = 12;

pub const EXIT_USAGE: i32 = 100;

fn main(){
  let argv: Vec<String> = env::args().collect();
  if argv.len() == 1{
    println!("usage: {} [expression]",argv[0]);
    std::process::exit(EXIT_USAGE);
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

