pub mod calculator {
    pub fn addition(number1: i32, number2: i32) -> i32 {
        number1 + number2
    }

    pub fn subtraction(number1: i32, number2: i32) -> i32 {
        number1 - number2
    }

    pub fn multiplication(number1: i32, number2: i32) -> i32 {
        number1 * number2
    }

    pub fn division(number1: i32, number2: i32) -> i32 {
        number1 / number2
    }
}

#[cfg(test)]
mod tests{
use crate::calculator::addition;
use crate::calculator::subtraction;
use crate::calculator::division;
use crate::calculator::multiplication;
#[test]
fn it_adds_four_and_five() {
    let result = addition(4, 5);
    assert_eq!(result, 9);
}
#[test]
fn it_adds_seven_and_three() {
    let result = addition(7, 3);
    assert_eq!(result, 10);
}
#[test]
#[should_panic]
fn it_adds_eleven_and_five() {
    let result = addition(11, 5);
    assert_eq!(result, 1000);
}
#[test]
fn it_calculate_substraction_of_hundred_and_eighty() {
    let result = subtraction(100, 80);
    assert_eq!(result, 20);
}
#[test]
#[should_panic]
fn it_calculate_substraction_of_seven_and_three() {
    let result = subtraction(7, 3);
    assert_eq!(result, 20);
}
#[test]
fn it_calculate_division_of_twentytwo_and_eleven() {
    let result = division(22, 11);
    assert_eq!(result, 2);
}
#[test]
#[should_panic]
fn it_calculate_division_of_ten_and_five() {
    let result = division(100, 80);
    assert_eq!(result, 20);
}
#[test]
fn it_multiplies_eleven_and_seven() {
    let result = multiplication(11, 7);
    assert_eq!(result, 77);
}
#[test]
#[should_panic]
fn it_multiplies_six_and_seven() {
    let result = multiplication(6, 7);
    assert_eq!(result, 20);
}
}