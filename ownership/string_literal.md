`let mut s = String::from("hello"); // s 는 String 타입`

`s.push_str(", world!"); // push_str()은 해당 스트링 리터럴을 스트링에 붙여줍니다.`

`println!("{}", s); // 이 부분이 "hello, world!"를 출력할 겁니다.`

스트링 리터럴의 경우, 변수의 값을 컴파일 시에 알 수 있다.
String 타입은 변경 가능하고 커질 수 있는 텍스트를 지원하기 위해 만들어졌다.
따라서, String 타입의 변수의 사용이 끝났을 때, OS에 메모리를 반납해야 한다.

메모리는 변수가 scope 를 벗어나는 순간 반납된다!
=> drop 함수 호출!
