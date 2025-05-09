use std::io;

pub fn calculate() {
    println!("Enter the number ");
    let mut number = String::new();
    io::stdin().read_line(&mut number).unwrap();
    let number: u32 = number.trim().parse().expect("not a number");

    println!("Choose the calculation method");
    println!("1: Addition");
    println!("2: Substraction");
    println!("3: Multiplication");
    println!("4: Division");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input: u32 = input.trim().parse().expect("not a number");

    match input {
        1 => addition(number),
        2 => substraction(number),
        3 => multiplication(number),
        4 => division(number),
        _ => println!("Thank you"),
    }

    fn addition(number: u32) {
        let first_number = number;
        let second_number = input_number();
        let total = first_number + second_number;
        println!("total number is {}", total);
    }
    fn substraction(number: u32) {
        let first_number = number;
        let second_number = input_number();
        let total = first_number + second_number;
        println!("total number is {}", total);
    }
    fn multiplication(number: u32) {
        let first_number = number;
        let second_number = input_number();
        let total = first_number + second_number;
        println!("total number is {}", total);
    }
    fn division(number: u32) {
        let first_number = number;
        let second_number = input_number();
        let total = first_number + second_number;
        println!("total number is {}", total);
    }

    fn input_number() -> u32 {
        println!("Please enter the second_number");
        let mut number = String::new();
        io::stdin().read_line(&mut number).unwrap();
        let number: u32 = number.trim().parse().expect("not a number");
        number
    }
}
