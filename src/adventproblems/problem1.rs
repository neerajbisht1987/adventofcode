use std::env;
use std::str;
//use std::iter::
pub fn inversecaptch()
{
    println!("Running problem 1..." );
    let arg:Vec<String> = env::args().collect();
    println!("sum={}",checkandsum(&arg[1]));
}


    fn checkandsum(s:& String)->u32
    {
        
        let strc = s.as_bytes();
        let mut val:u32=0;

        for i in 0..s.len() {
            if i == s.len()-1
            {  
                if strc[i] == strc[0]
                {
                    val += strc[i] as u32 - 48;
                }
            }
            else {
                if strc[i] == strc[i+1] {
                 val += strc[i] as u32 - 48;
                }
            }
        }
    val
    }

