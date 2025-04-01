[1mdiff --git a/solver.rs b/solver.rs[m
[1mindex d30bc01..0b9185a 100644[m
[1m--- a/solver.rs[m
[1m+++ b/solver.rs[m
[36m@@ -20,6 +20,9 @@[m [mpub fn solve(tokens: Vec<Token>) -> i64 {[m
 	//dbg!(&newtoks);[m
 	print_tokens(&newtoks);[m
 [m
[32m+[m[41m        [m
[32m+[m
[32m+[m
 	return 0;[m
 }[m
 [m
[36m@@ -127,4 +130,4 @@[m [mfn find_double_operators(tokens: &Vec<Token>){[m
 		}[m
 	}[m
 [m
[31m-}[m
\ No newline at end of file[m
[32m+[m[32m}[m
[1mdiff --git a/tokenize.rs b/tokenize.rs[m
[1mindex f47d8dc..812152d 100644[m
[1m--- a/tokenize.rs[m
[1m+++ b/tokenize.rs[m
[36m@@ -185,18 +185,19 @@[m [mpub fn add_token(expression: &mut Vec<Token>, id: &TokenType, input: &mut String[m
 pub fn token_to_string(token: &Token) -> String {[m
 	match token.id {[m
 [m
[31m-[m
[32m+[m[41m		[m
 		TokenType::Operation => {[m
[31m-			// rust is evil and does not let me use my enum here.[m
[31m-			match token.value {[m
[31m-				0 => {return "+".to_string(); }, // Operation::Add[m
[31m-				1 => {return "-".to_string(); }, // Operation::Sub[m
[31m-				2 => {return "*".to_string(); }, // Operation::Mul[m
[31m-				3 => {return "/".to_string(); }, // Operation::Div[m
[32m+[m			[32m#[allow(unreachable_patterns)][m
[32m+[m			[32mmatch unsafe { std::mem::transmute(token.value) } {[m
[32m+[m				[32mOperation::Add => {return "+".to_string(); },[m[41m  [m
[32m+[m				[32mOperation::Sub => {return "-".to_string(); },[m[41m  [m
[32m+[m				[32mOperation::Mul => {return "*".to_string(); },[m[41m  [m
[32m+[m				[32mOperation::Div => {return "/".to_string(); },[m[41m [m
[32m+[m				[32m#[allow(unreachable_code)][m
 				_ => { unimplemented!();	},[m
 			}[m
 		},[m
[31m-[m
[32m+[m[41m		[m
 [m
 		TokenType::OpenParen  => {return "(".to_string(); },[m
 		TokenType::CloseParen => {return ")".to_string(); },[m
