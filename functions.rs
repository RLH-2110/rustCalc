use tokenize::Token;
use tokenize::TokenType;
use tokenize::Operation;

/* !! ALWAYS UPDATE THE LOOKUP TABLE WHEN THE OPERATION ENUM IN TOKENIZE.RS IS UPDATED! !! */
type MathOp = fn(&i64, &i64) -> Result<i64,i32>;
const OP_LOOKUP: [MathOp;4] = [add,sub,mul,div];

pub fn calculate(a: &i64, b: &i64,op: &Operation) -> Result<Token,i32> {

  if *op as usize > Operation::Div as usize{
    println!("Operaton nr. {} is not implemented in the calculate function!",*op as usize);
    return Err(crate::EXIT_OP_NOT_IMPLEMENTED);
  }

  let result: i64;
  result = OP_LOOKUP[*op as usize](a,b)?;
  
  return Ok(Token {id: TokenType::Number, value: result, prio: 0});
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

  let fp: u8; unsafe { fp = crate::FIXED_POINT; }
  let scale: i64 = 2i64.pow(fp as u32);

  let ret = a.checked_mul(*b);
  if ret.is_none(){
    println!("OVERFLOW!");
    return Err(crate::EXIT_MATH_OVERFLOW);
  }
  return Ok(ret.unwrap()/scale);
}

fn div(a: &i64, b: &i64) -> Result<i64,i32>  {

  if *b == 0{
    println!("DIVISION BY ZERO!");
    return Err(crate::EXIT_DIV_BY_ZERO);
  }

  let fp: u8; unsafe { fp = crate::FIXED_POINT; }
  let scale: i64 = 2i64.pow(fp as u32);
  let la = *a * scale;
  let lb = *b;

  let ret = la.checked_div(lb);
  if ret.is_none(){
    println!("OVERFLOW!");
    return Err(crate::EXIT_MATH_OVERFLOW);
  }

  return Ok(ret.unwrap());
}