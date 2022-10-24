fn main() {
    // rust 는 기본적으로 불변성 변수이기 때문에 다시 선언하기 위해서는 mut 를 추가해야 한다. 
    // let mut x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");

    // shadowing 과 mut 의 차이점! => 
    // let x = 5;
    // let x = x + 1;
    // {
    //     let x = x * 2;
    //     println!("The value of x in the inner scope is: {x}");
    // }
    // println!("The value of x is {x}");

    // TODO: t와 f 에 _ 붙이는 이유!
    // if this is intentional, prefix it with an underscore: `_f`
    // let _t:bool = true;
    // let _f: bool = false;

    // 문자 타입
    // let c = 'z';
    // let z = 'z';
    // let heart_person = '🥰';

    // 튜플 변수 tup 를 x, y, z에 bind y만 출력!
    let tup = (500, 6.4, 1);
    let (_x, _y, _z) = tup;
    println!("The value of y is {}", _y);

    // 튜플 . 뒤에 원하는 인덱스를 지정하여 변수를 가져옴. 
    // let x: (i32, f64, u8) = (500, 6.4, 1);
    // let five_hundred = x.0;
    // let six_point_four = x.1;
    // let one = x.2;

    // 배열
    // let a = [1, 2, 3, 4, 5];

    // let first = a[0];
    // let second = a[1];

    // 유효하지 않은 배열 요소에 대한 접근 
    let a = [1, 2, 3, 4, 5];
    let index = 10;

    let element = a[index];

    println!("The value of element is: {}", element);

    

}
