use std::env;

mod tokenize;
mod solver;
mod functions;
mod conversion;

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
pub const EXIT_INVAL_OPERATION_ID: i32 = 13;
pub const EXIT_INPUT_OVERFLOW: i32 = 14;
pub const EXIT_HUMONGOUS_STRING: i32 = 15;
//pub const EXIT_NO_ZERO_FOR_FIXED_POINT_MATH: i32 = 16;

pub const EXIT_USAGE: i32 = 100;

static mut FIXED_POINT: u8 = 0; /* how many bits of fixed point */

fn main(){
  let argv: Vec<String> = env::args().collect();
  if argv.len() == 1{
    print_usage(&argv[0]);
  }

  let mut expression_index = 1;


  // check for fp flag
  if argv[1].chars().next().unwrap() == '-'{

    if argv[1].to_uppercase() == "-FP" && argv.len() > 3 { // 3 because [progamm name] [#fp] [number] [expression]

      let val = match argv[2].parse::<u8>(){
        Ok(val) => val,
        Err(_) => {print_fp_error();0},
      };

      if val > 18 {print_fp_error();} // limited to 18, so that the algorim to turn fixed point back to decimal does not overflow

      unsafe { FIXED_POINT = val; }
      expression_index+=2;
    }

    if argv[1].to_uppercase() == "-H" || argv[1].to_uppercase() == "-HELP" {
      print_usage(&argv[0]);
    }

    }

   



  let args: String = argv[expression_index..].join("");

  if args.len() > u32::MAX as usize{
    println!("INPUT TOO BIG, what the fuck are you doing?");
    std::process::exit(EXIT_HUMONGOUS_STRING);
  }

  let expression = match tokenize::parse(args){
    Ok(v) => v,
    Err(e) => {std::process::exit(e)},
  };
  let result = match solver::solve(expression){ 
    Ok(n) => n,    
    Err(e) => {std::process::exit(e);},
  };


  println!("{}", conversion::fp_to_string(result));
}  

// prints usage and exists
fn print_usage(progammname: &String){
  println!("usage: {} (arguments) [expression]\
    arguments:\n\
    \t -fp [number]\t - sets the amount of bits used for floating point (defaults to 0)\n\
    \t -help       \t - prints this menu"
    ,progammname);
  std::process::exit(EXIT_USAGE);
}

fn print_fp_error(){
   println!("Fixed point value must be between 0 and 18");
   std::process::exit(EXIT_USAGE);
}
