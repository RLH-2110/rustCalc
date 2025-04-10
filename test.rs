use std::process::Command;
use std::process::exit;

/* funtion by chatgpt, because I am to lazy to figure this out*/
fn run_calc(expr: &str) -> (i32, String) {
    let output = Command::new("./calc.elf")
        .arg(expr)
        .output()
        .expect("Failed to run calc.elf");

    let status = output.status.code().unwrap_or(-1);
    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    (status, stdout)
}
fn run_calc_fp(expr: &str, fp: u8) -> (i32, String) {
    let output = Command::new("./calc.elf")
        .arg("-fp")
        .arg(fp.to_string())
        .arg(expr)
        .output()
        .expect("Failed to run calc.elf");

    let status = output.status.code().unwrap_or(-1);
    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    (status, stdout)
}


// own code from here on out

/* 
 * test_num: number of the test.
 * expr: the expression given to calc.elf 
 * failing: true if we expect the result to be an error
 * result: the result we expect (only used when not a failing test)
 */
fn run_test(test_num: u32, expr: &str,failing: bool,result: i64){
    let (retcode, res) = run_calc(expr);
    

    // handle failing tests
    if retcode == 0 && failing == true{
        println!("test {test_num}. failed: expected ./calc.elf {expr} to fail, but no failure was found!");
        exit(1);
    }
    if retcode != 0 && failing == true{
        return;
    }




    if retcode != 0 {
        println!("test {test_num}. failed: expected ./calc.elf {expr} to return {result}, but got error {res}!");
        exit(1);
    }

    let parsed_res = res.parse::<i64>();
    match parsed_res {
        Err(_) => {println!("test {test_num}. failed: expected ./calc.elf {expr} to return {result}, but got {res}!");return;},
        Ok(_) => {},
    }

    if parsed_res.unwrap() == result{
        return;
    }else{
        println!("test {test_num}. failed: expected ./calc.elf {expr} to return {result}, but got {res}!");
        exit(1);
    }

}

/* 
 * test_num: number of the test.
 * expr: the expression given to calc.elf 
 * failing: true if we expect the result to be an error
 * result: the result we expect (only used when not a failing test)
 * fp: amount of bits used for fixed point
 */
fn run_test_fp(test_num: u32, expr: &str,failing: bool,result: f64, fp: u8){
    let (retcode, res) = run_calc_fp(expr,fp);
    

    // handle failing tests
    if retcode == 0 && failing == true{
        println!("test {test_num}. failed: expected ./calc.elf -fp {fp} {expr} to fail, but no failure was found!");
        exit(1);
    }
    if retcode != 0 && failing == true{
        return;
    }




    if retcode != 0 {
        println!("test {test_num}. failed: expected ./calc.elf -fp {fp} {expr} to return {result}, but got error {res}!");
        exit(1);
    }

    let parsed_res = res.parse::<f64>();
    match parsed_res {
        Err(_) => {println!("test {test_num}. failed: expected ./calc.elf -fp {fp} {expr} to return {result}, but got {res}!");return;},
        Ok(_) => {},
    }

    if parsed_res.unwrap() == result{
        return;
    }else{
        println!("test {test_num}. failed: expected ./calc.elf -fp {fp} {expr} to return {result}, but got {res}!");
        exit(1);
    }

}


fn main() {
    run_test(0,"1+1",false,2);
    run_test(1,"1++1",true,0);

    run_test(2,"abc",true,0);
    run_test(3,"1+c",true,0);

    run_test(4,"2+5",false,7);
    run_test(5,"2-5",false,-3);
    run_test(6,"2*5",false,10);
    run_test(7,"2/5",false,0);
    run_test(8,"2/0",true,0);
    run_test(9,"5/2",false,2);

    run_test(10,"5",false,5);
    run_test(11,"-5",false,-5);
    run_test(12,"(5)",false,5);
    run_test(13,"(-5)",false,-5);
    run_test(14,"-(5)",false,-5);
    run_test(15,"-(-5)",false,5);

    run_test(16,"2(5)",false,10);
    run_test(17,"-2(5)",false,-10);
    run_test(18,"-2(-5)",false,10);

    run_test(19,"4294967296",true,0); // bigger than u32!
    run_test(20,"999999999999999999999999999999999999999999999999999994294967296",true,0); // bigger than u32!
    run_test(21,"4294967295+4294967295",false,8589934590);
    run_test(22,"0-4294967295 ",false,-4294967295);
    run_test(23,"4294967295*4294967295 ",true,0); // overflow
    run_test(24,"4294967295/1 ",false,4294967295);
    
    run_test(25,"9(+5)",true,0); // make sure it fails correctly
    run_test(26,"9(*5)",true,0);
    run_test(27,"1(2)(3)",false,6);
    run_test(28,"()",true,0); 
    run_test(29,"2*(5)",false,10);
    run_test(30,"2*(5+ *7)",true,0);

    run_test(31,"9(2))",true,0);
    run_test(32,"9((2)",true,0);
    run_test(33,"1+3*4/2",false,7);
    run_test(34,"2(5*(1+1))",false,20);

    run_test_fp(35,"1.5",false,1.5,1);
    run_test_fp(36,"1.5+1.5",false,3.0,1);
    run_test_fp(37,"1.49",false,1.0,1);
    run_test_fp(38,"1.99",false,1.5,1);

    run_test_fp(39,"1..5",true,0.0,1);
    run_test_fp(40,"...",true,0.0,1);
    run_test_fp(41,".",true,0.0,1);
    run_test_fp(42,".5",false,0.5,1);
    run_test_fp(43,"5.",true,0.0,1);

    run_test_fp(44,"2*0.5",false,1.0,1);
    
    run_test_fp(45,"1.99",false,1.75,2);
    run_test_fp(46,"1.5+1.4",false,2.75,2);
    run_test_fp(47,"1.5*1.5",false,2.25,2);
    run_test_fp(48,"2/0.125",false,16.0,3);
    run_test_fp(49,"2.25/2",false,1.0,2);
    run_test_fp(50,"2.25/2",false,1.125,3);
    run_test_fp(51,"2.25*0.5",false,1.0,2);
    run_test_fp(52,"2.25*0.5",false,1.125,3);
    run_test_fp(53,"9999999999999999999999999999999999999999999999999999*0.5",true,0.0,3);
    run_test_fp(54,"1.9999999999999999999999999999999999999999999999999999*0.5",true,0.0,3);
    run_test_fp(55,"999999999999999999999999999999999999.9999999999999999999999999999999999999999999999999999*0.5",true,0.0,3);
    run_test_fp(56,"999999999999999999999999999999999999.999999999999999999999999999999.9999999999999999999999*0.5",true,0.0,3);
    run_test_fp(57,"0.5+.+0.5",true,0.0,3);

    println!("add more tests!");

   println!("all tests successfull!");
}
