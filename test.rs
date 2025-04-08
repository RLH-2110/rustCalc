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

    if res.parse::<i64>().unwrap() == result{
        return;
    }else{
        println!("test {test_num}. failed: expected ./calc.elf {expr} to return {result}, but got {res}!");
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
    run_test(20,"4294967295+4294967295",false,8589934590);
    run_test(21,"0-4294967295 ",false,-4294967295);
    run_test(22,"4294967295*4294967295 ",true,0); // overflow
    run_test(23,"4294967295/1 ",false,4294967295);
    

    println!("add more tests!");

   println!("all tests successfull!");
}
