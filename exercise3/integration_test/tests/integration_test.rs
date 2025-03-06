use integration_test::calculator::addition;
use integration_test::calculator::subtraction;
use integration_test::calculator::division;
use integration_test::calculator::multiplication;

#[test]
fn it_should_check_addition_succeedes() {
    let result = addition(4, 1);
    assert_eq!(result, 5);
}
#[test]
#[should_panic]
fn it_should_check_addition_fails () {
    let result = addition(4, 1);
    assert_eq!(result, 59);
}
#[test]
fn it_should_check_subtraction_succeedes() {
    let result = subtraction(44, 4);
    assert_eq!(result, 40);
}
#[test]
#[should_panic]
fn it_should_check_subtraction_fails() {
    let result = subtraction(456, 1);
    assert_eq!(result, 59);
}
#[test]
fn it_should_check_multiplicaiton_succeedes() {
    let result = multiplication(14, 2);
    assert_eq!(result, 28);
}

#[test]
#[should_panic]
fn it_should_check_multiplication_fails() {
    let result = multiplication(4, 1);
    assert_eq!(result, 500);
}
#[test]
fn it_should_check_division_succeedes() {
    let result = division(14, 2);
    assert_eq!(result, 7);
}
#[test]
#[should_panic]
fn it_should_check_division_fails() {
    let result = division(4, 0);
    assert_eq!(result, 90);
}