### Ownership

아래 s 는 String 타입이다.

`let mut s = String::from("hello");`

push_str() 은 해당 스트링 리터럴을 스트링에 붙여준다.

`s.push_str(", world!");`

expected hello world!
`println!("{}", s);`

스트링 리터럴의 경우, 변수의 값을 컴파일 시에 알 수 있다.
String 타입은 변경 가능하고 커질 수 있는 텍스트를 지원하기 위해 만들어졌다.
따라서, String 타입의 변수의 사용이 끝났을 때, OS에 메모리를 반납해야 한다.

메모리는 변수가 scope 를 벗어나는 순간 반납된다!
=> drop 함수 호출!

### Move

다음과 같은 변수 선언이 있다고 했을 때, x와 y 라는 변수를 선언하였고 그 값은 5로 같다.
`let x = 5;`
`let y = x;`

하지만 아래 예시에서는 위 코드와 다르게 동작한다.

`let s1 = String::from("hello");`
`let s2 = s1;`

이 에러를 뱉으면서!
=> move occurs because `s1` has type `std::string::String`, which does not implement the `Copy` trait

러스트는 절대 혼자서 깊은 복사해놓지 않기 때문에 s1 은 s2 로 move 되었고 s2에 값을 복사할 수 없다.
아니 복사할 수 있는 방법은 있다. clone 이라는 method 를 사용하면 할 수 있다.

`let s1 = String::from("hello);`
`let s2 = s1.clone();`

### Copy and Drop

i32 타입의 변수는 함수 스코프 안에 들어가도 복사가 가능한 반면,
String 타입의 변수는 스코프 안에 들어가면 String 타입의 변수가 정의된 바깥 스코프에서는 Drop 된다.
