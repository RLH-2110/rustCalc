
use tokenize::Token;
use tokenize::TokenType;
use tokenize::Operation;
use functions::calculate;


#[allow(unused_imports)]
use tokenize::print_tokens;

/* solves an expression and returns the result as an integer
 *
 * Vec<Token> tokens: the tokes with hte expression we want to solve
 *
 * returns the result as an result<i64,i32>
 * the error value is the progamm exit code.
 * the Ok value is the result fo the calulation
 * 
 */
pub fn solve(tokens: Vec<Token>) -> Result<i64,i32> {

  if tokens.len() == 0 {
    println!("Something went horribly wrong! there are no tokens!");
    return Err(crate::EXIT_NO_TOKS);
  }

  // remove unary -
  let mut newtoks = match remove_unary_minus(tokens){
    Ok(v) => v,
    Err(e) => {return Err(e);},
  };
  
  // detect leading operators
  if newtoks[0].id == TokenType::Operation {
    println!("leading operators at the start of an expression are not valid!");
    return Err(crate::EXIT_LEADING_OP);
  }

  // detect trailing operators
  if newtoks[newtoks.len()-1].id == TokenType::Operation {
    println!("trailing operators at the end of an expression are not valid!");
    return Err(crate::EXIT_TRAILING_OP);
  }

  // check for duplicate operators. something like "1++1"
  if find_double_operators(&newtoks) { return Err(crate::EXIT_DOUBLE_OPS); }  

  if has_other_unary(&newtoks) { println!("Only Minus is allowed as unary Operator!"); return Err(crate::EXIT_INVAL_UNARY);}

  let mut breaker: u16 = u16::MAX;
  let mut toks = newtoks;

  // as long as we have more than 1 token   (breaker is used to check for potental infinite loops)
  loop{
    if toks.len() == 1 {break};
    if breaker == 0 {break;}
    breaker -= 1;
  
    newtoks = Vec::new();
    let mut i = 0;

    /* for every token*/
    loop{
      if i >= toks.len() {break;}
      newtoks.push(toks[i]);

      // check if its like this: (Operator or nothing) Number Operator Number (Operator or nothing)
      // where all the operators have the same or priority or the outer operators have a lower priority
      // if the pattern is not found, go to next itteration
      if toks[i].id != TokenType::Operation {i+=1;continue;}
      if peek(&i,-1,&toks).is_some() && peek(&i,-1,&toks).unwrap().id != TokenType::Number {i+=1;continue;}
      if peek(&i, 1,&toks).is_some() && peek(&i, 1,&toks).unwrap().id != TokenType::Number {i+=1;continue;}
      if peek(&i,-2,&toks).is_some() && peek(&i,-2,&toks).unwrap().id == TokenType::Operation && peek(&i,-2,&toks).unwrap().prio > toks[i].prio {i+=1;continue;}
      if peek(&i, 2,&toks).is_some() && peek(&i, 2,&toks).unwrap().id == TokenType::Operation && peek(&i, 2,&toks).unwrap().prio > toks[i].prio {i+=1;continue;}
  
      // now we know that we are an operator, that our neibours are numbers, and the adjatent operators have the same or a lower priority
       
      let op = newtoks.pop().unwrap(); // we saved the operator, but we dont want to save it anymroe.
      let num = newtoks.pop().unwrap(); // we saved the number, but we dont want to save it anymore.

      // do the calulation
      let result: Result<Token,i32>;
      unsafe { result = calculate(&num.value,&peek(&i, 1,&toks).unwrap().value, &std::mem::transmute(op.value)); }
      
      match result{
        Ok(val) => {newtoks.push(val);},
        Err(e) => {return Err(e)},
      }

      i+=1; //the next token is a number, but we already delt with it, so we skip it.
      i+=1; // incement for the loop.
    }


    toks = remove_solved_parentesis(newtoks);
  }




  if toks.len() != 1{
    println!("Progamm error: caclulation took too long!");
    return Err(crate::EXIT_INFINITE_LOOP);
  }

  return Ok(toks[0].value);
}


/* peeks in any direction in a vector of tokens and either returns that token, or None
 *
 * &usize           i:      the current index we are at in the vector
 * i64              amount: the amount we want to peek ahead or behind
 * &'a Vec<Token>   vec:    the list of tokens we want to peek into
 *
 * returns an option that either contains the token at the positon we peeked,
 * if we peek out of bounds, then the option is None
 */
fn peek<'a>(i: &usize, amount: i64, vec: &'a Vec<Token>) -> Option<&'a Token>{

  let index: i64 = *i as i64;

  if index + amount < 0 { return None; }
  if index + amount >= vec.len() as i64 { return None; }

  return Some(&vec[(index+amount) as usize]);
}

/* removes unary minus operators from a vector of tokens, and replaces the numbers where the
 * operators where found with negative numbers
 *
 * Vec<Token> tokens: the vector of tokens we want to remove the unary minuses.
 *
 * returns a result with either vector of tokens with no unary minus operators,
 * or an i32 that contains an exit code
 */
fn remove_unary_minus(tokens: Vec<Token>) -> Result<Vec<Token>,i32> {

  let mut toks = tokens.clone();
  let mut found_unary: bool;

  loop{
    let mut newtoks = Vec::<Token>::new();
    found_unary = false;

    let mut i = 0;
    loop{
      if i >= toks.len(){
        break;
      }

      // if we are we an operator
      if toks[i].id == TokenType::Operation && toks[i].value == Operation::Sub as i64 {
        let ntok = peek(&i,1,&toks);

        if ntok.is_none() {
          break;
        }

        let ptok = peek(&i,-1,&toks);

        // if previous token is an operator or open Parentesis, or there is no previous token, and the next token is not an open parentesis
        if ptok.is_none() || ptok.unwrap().id == TokenType::Operation || ptok.unwrap().id == TokenType::OpenParen || ntok.unwrap().id == TokenType::OpenParen {
          found_unary = true;
          match ntok.unwrap().id {
            TokenType::Number => 
              {
                toks[i+1].value = 0-toks[i+1].value;
                newtoks.push(toks[i+1]); // push the negated number
                i+=2; 
              },
            TokenType::OpenParen => 
              { 
                newtoks.push(toks[i+1]); // push the "("
                i+=2;

                loop {
                  if i >= toks.len() || toks[i].id == TokenType::CloseParen { break; }
                  if toks[i].id == TokenType::Number { toks[i].value = 0-toks[i].value; } // negate all numbers
                  newtoks.push(toks[i]); // push element
                  i+=1;
                }

                newtoks.push(toks[i]); // push the ")"
                i+=1;
              },
            _ => 
              {  
                println!("MUTLIPLE OPERATIONS AFTER EACH OTHER WITH NO NUMBER!");
                return Err(crate::EXIT_DOUBLE_OPS);
              },
          }


        }else {
          newtoks.push(toks[i]);
          i += 1;
        }
        
      }else{
        newtoks.push(toks[i]);
        i+=1;
      }

    }

    toks = newtoks;
    if found_unary == false {
      return Ok(toks);
    }
  }
}

fn has_other_unary(tokens: &Vec<Token>) -> bool{
  let mut i = 0;
  for token in tokens{
    if token.id == TokenType::OpenParen {
      if  peek(&i, 1,&tokens).is_some() && peek(&i, 1,&tokens).unwrap().id == TokenType::Operation {
        return true;
      }
    }
    i += 1;
  }

  return false;
}

/*find double opeprators (like 1++1) and returns true if they are found.
 *
 * &Vec<Token> tokens: the lsit of tokens where we want to check for double operators
 *
 *  returns true if double operators where found, false if not.
 */
fn find_double_operators(tokens: &Vec<Token>) -> bool{

  let mut had_op = false;

  for token in tokens{
    if token.id == TokenType::Operation {
      if had_op {
        println!("Double operators found!");
        return true;
      }else{
        had_op = true;
      }
    }else{
      had_op = false;
    }
  }
  return false;
}


/* removes solved parentesis like turning (5) into 5
 *
 * Vec<Token> tokens: the vector where we want to remvoe the parentesis.
 *
 */
fn remove_solved_parentesis(tokens: Vec<Token>) -> Vec<Token> {
    let mut new_tokens: Vec<Token> = Vec::new();
    
    let mut i = 0;

    loop{
      if i >= tokens.len() {break;}
      new_tokens.push(tokens[i]);

      if tokens[i].id != TokenType::Number {i+=1;continue;}
      if peek(&i,-1,&tokens).is_some() && peek(&i,-1,&tokens).unwrap().id != TokenType::OpenParen {i+=1;continue;}
      if peek(&i, 1,&tokens).is_some() && peek(&i, 1,&tokens).unwrap().id != TokenType::CloseParen {i+=1;continue;}
  
      // now we know that we are an Number, that our neibours are parentesis
     
      let num = new_tokens.pop().unwrap(); // we saved the number, but we need to pop another value
                                          // before it
      new_tokens.pop(); // we saved the parantesis in newTokens before, but we dont want to save it anymroe.
        
      // check if we need to add a multiplication

      if peek(&i,-2,&tokens).is_some() && peek(&i,-2,&tokens).unwrap().id == TokenType::Number {
        new_tokens.push(Token {id: TokenType::Operation, value: Operation::Mul as i64, prio: 1});
      }

      new_tokens.push(num); // put the number back
      
      i+=1; //the next token is a braket, but we already delt with it, so we skip it.
      i+=1; // incement for the loop.
    }

    return new_tokens;
}
