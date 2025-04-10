
use tokenize::Operation;


// converts number into Operation

pub fn op_from_num(num: i64) -> Result<Operation,i32> {
	if num > Operation::Div as i64 || num < 0{
		return Err(crate::EXIT_INVAL_OPERATION_ID);
	}

	unsafe { return Ok(std::mem::transmute(num)); }
}


pub fn string_to_fp(str: &String) -> Result<i64,i32>{

	let fp: u8;
	unsafe { fp = crate::FIXED_POINT; }

	let snum: Vec<String> = str.splitn(2,".").map(|s| s.to_string()).collect(); // thanks deepseek for figuring out the mapping bullshit.
	let mut upper: i64 = 0;
	let lower: i64;

	if snum[0] != "" {
		upper = match snum[0].parse::<u32>(){
        Ok(val) => val,
        Err(_) => { println!("Numbers must be nummbers between 0 and 4294967295!"); return Err(crate::EXIT_INPUT_OVERFLOW);},
      } as i64;
	}

	upper = checked_shl(upper,fp)?;

	if snum.len() == 1 || snum[1] == "" {
		return Ok(upper);
	}

	lower = string_make_fp(fp,&snum[1])?;

	return Ok(upper|lower);
} 

// helper for function above
fn string_make_fp(fp: u8, str: &String) -> Result<i64,i32> {
	debug_assert!(fp != 0);

	if str.len() > crate::MAX_FP_SIZE as usize{
		println!("A maximum of 18 digits are allowed after the decimal!");
		return Err(crate::EXIT_TOO_BIG_FLOAT);
	}
	let div_val: i64 = 10i64.pow(str.len() as u32);
	let mut result: i64 = 0;

	let mut input: i64 = match str.parse::<u32>(){
        Ok(v) => v,
        Err(_) => { println!("Numbers must be nummbers between 0 and 4294967295!"); return Err(crate::EXIT_INPUT_OVERFLOW);},
      } as i64; 

    let mut i = fp;
	loop {

		input*=2;
		if (input/div_val) > 0 {
			result+=1;
			input -= div_val;
		}


		i -= 1;
		if i == 0{
			break;
		}
		result = result << 1;
	}
	return Ok(result);
}

pub fn fp_to_string(num: i64) -> String{

	let fp: u8;
	unsafe { fp = crate::FIXED_POINT; }
	let mut string: String = (num >> fp).to_string();

	if fp == 0{
		return string;
	}

	string.push_str(".");

	let mut mask: i64 = 0;
	for _ in 0..fp {
		mask = mask << 1;
		mask += 1;
	}

	let divider:i64 = 1 << fp;
	let mut n: i64 =  num & mask;

	for _ in 0..fp {
		n *= 10;
		let digit: i64 = n / divider;
		string.push_str(&digit.to_string());
		n %= divider;

		if n == 0 {
			break;
		}
	}

	return string;

} 


// VAL MUST BE A u32 STORED INSIDE A i64!
// shifts the number left by amount
//
// val: u32 value stored as i64 that we want to shift
// amount: by how much we want to shfit left
//
// returns: result of either the i64 result, or EXIT_INPUT_OVERFLOW as i32 exit code
fn checked_shl(val: i64, amount: u8) -> Result<i64,i32>{
	let ret = val << amount;
	if val != ret >> amount{
		return Err(crate::EXIT_INPUT_OVERFLOW);
	}
	return Ok(ret);
}