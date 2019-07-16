use std::io;

fn main() {
    println!("guessing game");
    println!("please input a number");

    let mut guess=String::new();

    io::stdin().read_line(&mut guess)
                .expect("Failed to read line");

    println!("guess number {}",guess);
}
