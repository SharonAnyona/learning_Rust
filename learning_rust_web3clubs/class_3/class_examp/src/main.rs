fn main() {
    println!("Hello, world!");
    let var_1: [u32; 4]=[4,34,4,3];
   let sum = calc_sum(param_1: &var_1);
   println!("sum is {sum}");

   let mut stri_1:  String= my_string(par_1: &var_1);
   println!("sum is {sum}", stri_1);


    
}

//take in array reference
// loop through items
// retrun sum
fn calc_sum(param_1:&[u32]) -> u32{

    let mut param_2: i32 = 0;
    for elements:&u32 in param_1{

     param_2 = elements + param_2;

    }

    param_2;
}

// funtion that takes in a string
fn my_string(par_1:& mut String) -> & mut String{
    value.push str( string_1="have a good night");
    value
}
