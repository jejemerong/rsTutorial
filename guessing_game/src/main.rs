use std::io;
use rand::Rng; //랜덤수를 생성하기 위한 라이브러리
use std::cmp::Ordering; // 두 수 비교하는 라이브러리

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("Please input your guess!");

    // mut: 가변성 변수로 사용하기 위해!
    // let mut guess = String::new();

    // 사용자한테 인풋값 받기!
    // io::stdin().read_line(&mut guess).expect("Failed to read line");

    // expect => number 가 아닌 문자가 들어올 경우를 대비하여
    // let guess: u32 = guess.trim().parse().expect("Please type a number");

    // guess 가 secret_number 와 같아질 때까지 반복!
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        // let guess : u32 = guess.trim().parse().expect("Please type a number!");
        // guess type 이 sting 이냐 아니냐
        let guess : u32 = match guess.trim().parse(){
            Ok(num) => num, 
            Err(_) => continue, 
        };
        println!("You guessed: {guess}");

        // 두 수 비교
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! The secret number is: {secret_number}");
                break;
            }
        }
    }
}
