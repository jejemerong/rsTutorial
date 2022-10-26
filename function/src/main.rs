fn iter (){
    loop {
        println!("again!");
    }
}

fn iter_1 (){
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }
    println!("LIFTOFF!!!");
}

fn iter_2 (){
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index = index + 1;
    }
}

fn for_iter() {
    let a = [10, 20, 30, 40, 50];

    // 배열의 길이를 따로 지정하지 않아도 배열 a 에 대해서 반복문 실행
    // for el in arr.iter(){}
    for element in a.iter(){
        println!("the value is: {}", element);
    }
}

// TODO: 갑자기 rev?!
// range 를 역순하여 반복하는 rev!!
fn main() {
    // (1..4) => 1<=x<4 인가?!
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
