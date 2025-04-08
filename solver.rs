
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
    return Err(1);
  }

  // remove unary -
  let mut newtoks = match remove_unary_minus(tokens){
    Ok(v) => v,
    Err(e) => {return Err(e);},
  };
        
  if newtoks[0].id == TokenType::Operation {
    println!("leading operators at the start of an expression are not valid!");
    return Err(1);
  }
  if newtoks[newtoks.len()-1].id == TokenType::Operation {
    println!("trailing operators at the end of an expression are not valid!");
    return Err(1);
  }

  if find_double_operators(&newtoks) { return Err(1); }    
  let mut breaker: u16 = u16::MAX;
  let mut toks = newtoks;

  loop{
    if toks.len() == 1 {break};
    if breaker == 0 {break;}
    breaker -= 1;
  
    newtoks = Vec::new();
    let mut i = 0;
    loop{
      if i >= toks.len() {break;}
      newtoks.push(toks[i]);

      if toks[i].id != TokenType::Operation {i+=1;continue;}
      if peek(&i,-1,&toks).is_some() && peek(&i,-1,&toks).unwrap().id != TokenType::Number {i+=1;continue;}
      if peek(&i, 1,&toks).is_some() && peek(&i, 1,&toks).unwrap().id != TokenType::Number {i+=1;continue;}
      if peek(&i,-2,&toks).is_some() && peek(&i,-2,&toks).unwrap().id == TokenType::Operation && peek(&i,-2,&toks).unwrap().prio > toks[i].prio {i+=1;continue;}
      if peek(&i, 2,&toks).is_some() && peek(&i, 2,&toks).unwrap().id == TokenType::Operation && peek(&i, 2,&toks).unwrap().prio > toks[i].prio {i+=1;continue;}
  
      // now we know that we are an operator, that our neibours are numbers, and the adjatent operators have a lower priority
       
      let op = newtoks.pop().unwrap(); // we saved the operator, but we dont want to save it anymroe.
      let num = newtoks.pop().unwrap(); // we saved the number, but we dont want to save it anymore.

      unsafe { newtoks.push(calculate(&num.value,&peek(&i, 1,&toks).unwrap().value, &std::mem::transmute(op.value))); }

      i+=1; //the next token is a number, but we already delt with it, so we skip it.
      i+=1; // incement for the loop.
    }


    toks = remove_solved_parentesis(newtoks);
  }

  //dbg!(&toks);
  //print_tokens(&toks);

  if toks.len() != 1{
    println!("Progamm error: caclulation took too long!");
    return Err(2);
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

      if toks[i].id == TokenType::Operation && toks[i].value == Operation::Sub as i64 {
        let ntok = peek(&i,1,&toks);

        if ntok.is_none() {
          break;
        }

        let ptok = peek(&i,-1,&toks);

        if ptok.is_none() || ptok.unwrap().id == TokenType::Operation || ntok.unwrap().id == TokenType::OpenParen {

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
                return Err(1);
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
  //TODO HERE!
  return tokens;
}
