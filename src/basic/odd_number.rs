use std::io;
///
pub fn odd_number() {
    println!("Please enter the number till you want the print the odd number");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let number: u32 = input.trim().parse().expect("Please type a number!");

    for i in 0..number {
        if !i % 2 == 0 {
            println!("the number is first {}", i);
            let number = i + 1;
            println!("the number is {}", number);
        }
    }

    // We can also use the step by function of the rust
    // for i in (0..number).step_by(2) {
    //     println!("the number is {}", i + 1);
    // }
}
