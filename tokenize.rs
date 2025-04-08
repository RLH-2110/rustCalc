#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Token {
  pub id: TokenType,
  pub value: i64, // saves either the value for a number, or the operation for operations. In
                        // case of operation, the Operation enum is used.
  pub prio: u8,   // is used to find priorities for operations. normally + has 0 and * has 1
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum TokenType{
  Number,
  Operation,
  OpenParen,
  CloseParen,

  None, /* used when saving what the previous token is, and there is no previous token*/
  Invalid,
}


/* !! ALWAYS UPDATE THE OPERATION ENUM WHEN THE LOOKUP TABLE IN FUNCTIONS.RS IS UPDATED! !! */
#[repr(i64)]
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Operation{
  Add,
  Sub,
  Mul,
  Div,
}


/* turns a string (without spaces or newlines!) into a list of tokens
 *
 *  String text: a string without whitespace
 *
 *  returns: an result with either a vector of tokens or an u32 exit code for the progamm
 *
 */
pub fn parse(text: String) -> Result<Vec<Token>,i32>{

  let mut expression: Vec<Token> = Vec::new();
  let mut input: String = String::with_capacity(8);
  let mut id = TokenType::None;
  let mut invalid_token = false; /*for handleing newlines for invalid token error prints*/
  let mut braket_count = 0;

  for c in text.chars(){

    // if number
    if c.is_digit(10){

      if id != TokenType::Number{
        if id != TokenType::None{
          if add_token(&mut expression,&id,&mut input) {return Err(4);}
        }
        
        input.push(c);
        id = TokenType::Number;
        continue;
      }
      input.push(c);
      continue;
    }


    // if op
    if "+-*/".contains(c){

      if id != TokenType::Operation{
        if id != TokenType::None{
          if add_token(&mut expression,&id,&mut input) {return Err(4);}
        }

        input.push(c);
        id = TokenType::Operation;
        continue;
      }
      if add_token(&mut expression,&id,&mut input) {return Err(4);}
      input.push(c);
      continue;
    
    }

    // if braket
    if c == '('{
      braket_count += 1;
      if id != TokenType::OpenParen  {
        if id != TokenType::None{
          if add_token(&mut expression,&id,&mut input) {return Err(4);}
        }

        input.push(c);
        id = TokenType::OpenParen;
        continue;
      }
      if add_token(&mut expression,&id,&mut input) {return Err(4);}
      input.push(c);
      continue;
    
    }
    if c == ')'{
      braket_count -= 1;

      if braket_count < 0 { // user entered more ) than possible
        println!("there are unopnened brakets!");
        return Err(4);
      }

      if id != TokenType::CloseParen {
        if id != TokenType::None{
          if add_token(&mut expression,&id,&mut input) {return Err(4);}
        }

        input.push(c);
        id = TokenType::CloseParen;
        continue;
      }
      if add_token(&mut expression,&id,&mut input) {return Err(4);}
      input.push(c);
      continue;
    
    }

    if c == ' '{ // ignore whitespace
      continue;
    }

    // if other
    {
      
      if id != TokenType::Invalid{
        if invalid_token { println!(""); } /*new invalid token error message*/
        print!("Unknown Token: ");

        if id != TokenType::None{
          if add_token(&mut expression,&id,&mut input) {return Err(4);}
        }

        print!("{c}");
        input.push(c);
        id = TokenType::Invalid;

        invalid_token = true;
        continue;
      }
      input.push(c);
      print!("{c}");
      continue;
    }

  }
  if add_token(&mut expression,&id,&mut input) {return Err(4);}
  if invalid_token { println!(""); return Err(2); }

  if braket_count != 0{
    if braket_count > 0{
      println!("there are {braket_count} unclosed brakets!");
    }else{
      // will never execute, since we check for this when we parse ')'
      // I left it in in case I remove it. "doppelt hält besser."
      println!("there are {} unopnened brakets!",0-braket_count);
    }
    return Err(5);
  }

  return Ok(expression);
}



/* creates a token with a token type and a string
 *
 * &mut Vec<Token>  expression: the vector of tokens we want to add the token to
 * &TokenType       id:         the TokenType we want to add
 * &mut String      input:      a string with the data for the token, usually a number or an operator, but it can aslo be
 *                              a parantesis, though in that case the input is unused.
 *
 * returns false if no error occured, returns true if an error occured
 */
pub fn add_token(expression: &mut Vec<Token>, id: &TokenType, input: &mut String) -> bool{


  match *id{
    TokenType::Operation => {
      
      let (val,pri) = match input.as_str(){
        "+" => (Operation::Add as i64,0),
        "-" => (Operation::Sub as i64,0),
        "*" => (Operation::Mul as i64,1),
        "/" => (Operation::Div as i64,1),
        _   => (Operation::Add as i64,0),
      };

      expression.push(Token {id: *id, value: val, prio: pri});
    },

    TokenType::Number => {
      let val = match input.parse::<u32>(){
        Ok(val) => val,
        Err(_) => { println!("Numbers must be nummbers between 0 and 4294967295!"); return true;},
      };
      expression.push(Token {id: *id, value: val as i64, prio: 0});
    },

    _ => {
      expression.push(Token {id: *id, value: 0, prio: 0});
    }
  }


  input.clear();
  return false;
}



/*Turns a token into a string
 *
 * &token token: the token to turn into a string
 *
 * returns a string representation of the token, which will NOT have a newline at the end.
 *
 * */
#[allow(dead_code)]
pub fn token_to_string(token: &Token) -> String {
  match token.id {

      TokenType::Operation => {
        if token.value > Operation::Div as i64 {
        unimplemented!();
      }
  
      match unsafe { std::mem::transmute(token.value) } {
        Operation::Add => {return "+".to_string(); },  
        Operation::Sub => {return "-".to_string(); },  
        Operation::Mul => {return "*".to_string(); },  
        Operation::Div => {return "/".to_string(); }, 
      }
    },
    

    TokenType::OpenParen  => {return "(".to_string(); },
    TokenType::CloseParen => {return ")".to_string(); },
    
    TokenType::Number => {return token.value.to_string(); },

    _ => {  unimplemented!();  }
  }
}

/* prints all the tokens in a vector
 * also prints a newline at the end.
 *
 * &Vec<Token> tokens: the vector of tokens to print
 */
 #[allow(dead_code)]
pub fn print_tokens(tokens: &Vec<Token>){
  for token in tokens {
    print!("{}",token_to_string(&token));
  }
  print!("\n");
}
