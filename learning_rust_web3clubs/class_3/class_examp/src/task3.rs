fn main(){
    let var_1 = [1.2,3.4,34.];
   let result_1= my_aray(var_1);
   println!("The result is {}"), result_1;
}
fn my_aray(par_1: &[f32])-> f32{
    let mut index = 0.0;
    let mut lenght = par_1.len();
    let mut totals = 0;
    while index < lenght{
        //index += 1
        index = index + 1;
        total = totals + par_1[index];
}
return totals
}