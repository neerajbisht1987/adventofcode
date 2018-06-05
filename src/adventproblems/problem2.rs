use std::env;
//use std::num;
//use std::str;

pub fn corrupt_checksum() 
{
    println!("Running problem 2..." );
    let _arg:Vec<String> = env::args().skip(1).collect();

    let argument= take_argument(_arg);
    let vec_of_vec = parse_and_find_min_max_diff(&argument);
    for  vec_list in vec_of_vec 
    {
        for  val in vec_list.iter()
        {
            println!("{}",val );

        }
    }
    
   
}
fn parse_and_find_min_max_diff(vec_vec_list :&Vec<Vec<i32>>) -> &Vec<Vec<i32>>
{ 
    
    let sum = 0;
    for  vec_list in vec_vec_list 
    { 
        let mut low:i32= <i32>::max_value();
        let mut high:i32= <i32>::min_value();
        for val in vec_list.iter()
        {
            if *val < low
            {
                low = *val;
            }
            if *val > high
            {
                high = *val;
            }
        }
        sum += high - log

    }
    vec_vec_list
}
fn take_argument(arg :Vec<String>) ->Vec<Vec<i32> >
{
    let _len= arg.len() as f32;
    let _length = _len.sqrt().abs() as i32;
    let _index =0;
    let mut vec_of_vec= Vec::new();
    let mut vec_list : Vec<i32> = Vec::new();
    for val in arg {
        let val = val.parse::<i32>().unwrap();
        vec_list.push(val);
        if vec_list.len() as i32 == _length
        {
            vec_of_vec.push(vec_list);
            vec_list = Vec::new();            
        }
    }

    // for  ref vec_list in &vec_of_vec 
    // {
    //     for  ref val in vec_list.iter()
    //     {
    //         println!("{}",val );

    //     }
    // }
        vec_of_vec

}