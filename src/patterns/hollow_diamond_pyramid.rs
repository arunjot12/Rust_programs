pub fn hollow_diamond() {
    for i in 1..=5 {
        for _ in i..=5 {
            print!(" ");
        }
        for k in 1..=2 * i - 1 {
            if k == 1 || k == 2 * i - 1 {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    for i in (1..=5).rev() {
        for _ in i..=5 {
            print!(" ");
        }
        for k in 1..=2 * i - 1 {
            if k == 1 || k == 2 * i - 1 {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
