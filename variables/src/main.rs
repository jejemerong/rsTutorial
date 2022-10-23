fn main() {
    // rust 는 기본적으로 불변성 변수이기 때문에 다시 선언하기 위해서는 mut 를 추가해야 한다. 
    // let mut x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");

    // shadowing 과 mut 의 차이점!
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is {x}");
}
