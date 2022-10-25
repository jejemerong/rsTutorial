fn main() {
    println!("Hello, world!");
    another_function(5, 6);

    // 함수는 표현식으로 작성해야 함.
    // let x = (let y = 6);
    
    // let x = 5;
    // let x = five();
    let x = plus_one(5);
    let y = {
        // 그리고 또 하나 새로 안 것: 함수는 호이스팅이 되지만, 변수는 안된다!
        // let x = 3;
        // 함수 표현식 마지막에 리턴부에는 ;을 붙이지 않는다!
        x + 1
    };
    println!("the value of y is: {}", y);

    // 조건문
        let number = 6;
        if number < 5 {
            println!("condition was true");
        } else{
            println!("condition was false");
        }
    // 조건문 에러 상황: expected `bool`, found integer
    // bool 타입이 아닌 것을 자동으로 bool 로 변환하지 않기 때문에 
    // 0 과 1 인 것도 false, true 가 될 수 없다. 
    // if number {
    //     println!("the number is three!");
    // }
    // 아래와 같이 바꿔주는 것이 정확하다. 
    // if number != 0 {
    //     println!("number was something other than zero.");
    // }
    // if number %4 == 0 {
    //     println!("number is divisible by 4");
    // }else if number %3 == 0 {
    //     println!("number is divisible by 3");
    // }else if number %2 == 0{
    //     println!("number is divisible by 2");
    // }else {
    //     println!("number is not divisible by 4, 3 or 2");
    // }

    let condition = true;
    // 조건문의 표현식 리턴값이 let 변수에 담기게 된다. 
    // let result = if condition {
    //     5
    // }else {
    //     6
    // };

    // if와 else 각각 return 값의 타입은 같아야 한다!
    // expected integer, found `&str`
    let result = if condition {
        5
    }else{
        "six"
    };
    println!("the value of result is: {}", result);
}

fn another_function(x: i32, y: i32){
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

// fn five() -> i32{
//     5
// }

fn plus_one(x: i32) -> i32 {
    x + 1
}

