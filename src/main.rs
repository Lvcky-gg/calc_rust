use std::env::{args, Args};
mod operators;
//cargo run -- lvcky
//Args = Args { inner: ["target/debug/calculator", "--freecodecamp"] }
// cargo run -- 1 + 2

fn main() {
    let mut args = args();
    //grabs the children of Args, and unwraps it
    let first = args.nth(1).unwrap();
    //creates an "object", accesses it, converts to chars, calls the iterator for specific, then unwraps
    let operator = args.nth(0).unwrap().chars().next().unwrap();
    let second = args.nth(0).unwrap();
    //type declaration....float
    let first_number: f32 = first.parse().unwrap();
    //turbo fish syntax to accomplish type dec
    let second_number = second.parse::<f32>().unwrap();
    let result = operators::operate(operator, first_number, second_number);

    println!("{:?}", operators::output(first_number, operator, second_number, result));
}
// // -> return type
// // implicit return in conditionals!!!!!!!
// fn operate(operator: char, first_number: f32, second_number: f32)->f32{
//     match operator {
//         '+' => first_number + second_number,
//         '-' => first_number - second_number,
//         '/' => first_number / second_number,
//         '*'| 'x' | 'X' => first_number * second_number,
//         _ => panic!("Panic. Invalid Operator used!!!")
//     }
//     //same as this
//     // if operator == '+'{
//     // first_number + second_number
//     // }else if operator == '-'{
//     // first_number - second_number
//     // }else if operator == '/'{
//     // first_number / second_number
//     // }else if operator  == '*' {
//     // first_number * second_number
//     // }else {
//     // 0.0
//     // }
// }

// fn output(first_number: f32, operator:char, second_number: f32, result: f32) -> String {
//     format!("{} {} {} = {}", first_number, operator, second_number, result)
// }




//what is a struct?
//there is None in rust


//Python compiler
//takes in each file as arg
//creates a file for each that replaces each bit of python code with rust
//
