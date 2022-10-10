use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess!");

    // mut: 가변성 변수로 사용하기 위해!
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("You guessed: {guess}");
}
