fn main() {
    let number1: i32 =12;
     let number2: i32 =12;

     sum(number1, number2);

     let x: i32 = 22;
     let y: i32 =32;
     sum_1(x, y);

}

fn sum(number1: i32, number2: i32){
     let number3:i32 = number1+number2;
     println!(" the sum of two integers is {}", number3);
}
fn sum_1(x: i32, y:i32)-> i32{
    let total_sum: i32 = x+y;
    total_sum // impisite return
    // return total_sum; //explisite return

}