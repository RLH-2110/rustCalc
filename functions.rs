use tokenize::Token;
use tokenize::TokenType;
use tokenize::Operation;

/* !! ALWAYS UPDATE THE LOOKUP TABLE WHEN THE OPERATION ENUM IN TOKENIZE.RS IS UPDATED! !! */
type MathOp = fn(&i64, &i64) -> Result<i64,i32>;
const OP_LOOKUP: [MathOp;4] = [add,sub,mul,div];

pub fn calculate(a: &i64, b: &i64,op: &Operation) -> Result<Token,i32> {
  let result: Result<i64,i32>;
  unsafe {result = OP_LOOKUP[std::mem::transmute::<Operation,usize>(*op)](a,b);}
  
  if result.is_err() {
    return Err(result.unwrap_err());
  }

  return Ok(Token {id: TokenType::Number, value: result.unwrap(), prio: 0});
}

fn add(a: &i64, b: &i64) -> Result<i64,i32> {

  let ret = a.checked_add(*b);
  if ret.is_none(){
    println!("OVERFLOW!");
    return Err(crate::EXIT_MATH_OVERFLOW);
  }
  return Ok(ret.unwrap());
}

fn sub(a: &i64, b: &i64) -> Result<i64,i32>  {

  let ret = a.checked_sub(*b);
  if ret.is_none(){
    println!("OVERFLOW!");
    return Err(crate::EXIT_MATH_OVERFLOW);
  }
  return Ok(ret.unwrap());
}

fn mul(a: &i64, b: &i64) -> Result<i64,i32>  {

  let ret = a.checked_mul(*b);
  if ret.is_none(){
    println!("OVERFLOW!");
    return Err(crate::EXIT_MATH_OVERFLOW);
  }
  return Ok(ret.unwrap());
}

fn div(a: &i64, b: &i64) -> Result<i64,i32>  {

  if *b == 0{
    println!("DIVISION BY ZERO!");
    return Err(crate::EXIT_DIV_BY_ZERO);
  }

  let ret = a.checked_div(*b);
  if ret.is_none(){
    println!("OVERFLOW!");
    return Err(crate::EXIT_MATH_OVERFLOW);
  }
  return Ok(ret.unwrap());
}