// -> return type
// implicit return in conditionals!!!!!!!
pub fn operate(operator: char, first_number: f32, second_number: f32)->f32{
    match operator {
        '+' => first_number + second_number,
        '-' => first_number - second_number,
        '/' => first_number / second_number,
        '*'| 'x' | 'X' => first_number * second_number,
        _ => panic!("Panic. Invalid Operator used!!!")
    }
    //same as this
    // if operator == '+'{
    // first_number + second_number
    // }else if operator == '-'{
    // first_number - second_number
    // }else if operator == '/'{
    // first_number / second_number
    // }else if operator  == '*' {
    // first_number * second_number
    // }else {
    // 0.0
    // }
}

pub fn output(first_number: f32, operator:char, second_number: f32, result: f32) -> String {
    format!("{} {} {} = {}", first_number, operator, second_number, result)
}