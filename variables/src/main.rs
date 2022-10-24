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
    let _t:bool = true;
    let _f: bool = false;

    // 문자 타입
    let c = 'z';
    let z = 'z';
    let heart_person = '🥰';
}
