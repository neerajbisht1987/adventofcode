use std::env;
//use std::str;
//use std::iter::
pub fn inversecaptch()
{
    println!("Running problem 1..." );
    let arg:Vec<String> = env::args().collect();
    println!("sum={}",checkandsum(&arg[1]));
}


fn checkandsum(s:& String)->u32
{
    let strc: Vec<char> = s.chars().collect();
    let mut val:u32=0;

    for i in 0..strc.len() {
        if i == strc.len()-1
        {  
            
            if strc[i] == strc[0]
            {
                val += strc[i].to_digit(10).unwrap();
            }
        }
        else {
            if strc[i] == strc[i+1] {
                val += strc[i].to_digit(10).unwrap();
            }
        }
    }
val
}

