// 연습과정에서 주의를 산만하게 하지 않도록 일부 경고를 무시합니다.
#![allow(unused_mut, unused_variables)]

fn main() {
    // 이 멋진 코드는 첫 번째 인수를 String으로 받거나, 프로그램에 인수가 제공되지 않았을 경우
    // 사용법을 출력하고 종료합니다.
    let mut arg: String = std::env::args().nth(1).unwrap_or_else(|| {
        println!("이 프로그램에 인수를 제공하세요.");
        std::process::exit(-1);
    });

    // 1. String의 참조를 받고 아무것도 반환하지 않지만, String의 내용이 복수인지 단수인지 출력하는
    // `inspect` 함수를 작성하세요. 그런 다음 아래 코드를 주석 해제하고 `cargo run apple` 및 `cargo run apples`로 실행하세요.
    // 힌트: String 참조에 `.ends_with("s")`를 사용하세요.
    //
    inspect(&arg);

    // 2. String의 *가변* 참조를 받아서 String이 "s"로 끝나지 않았다면 "s"를 추가하는
    // `change` 함수를 작성하세요. 그런 다음 아래 코드를 주석 해제하고 `cargo run apple`로 실행하세요.
    // 힌트: 가변 String 참조에 `.push_str("s")`를 사용하여 "s"를 추가하세요.
    //
    change(&mut arg);
    println!("저는 많은 {}를 갖고 있습니다", arg);

    // 3. String의 소유권을 받아들이고(소비하는) String이 "b"로 시작하고 "a"를 포함하는지 여부를 나타내는
    // bool 값을 반환하는 `eat` 함수를 작성하세요.
    // 힌트 1: `.starts_with("b")`와 `.contains("a")`를 사용하세요.
    // 힌트 2: `&&`는 불리언 "AND" 연산자입니다.
    //
    if eat(arg) {
        println!("바나나일 수도 있습니다");
    } else {
        println!("바나나가 아닙니다");
    }

    // "boat", "banana", "grapes"를 인수로 실행해보세요 :-)

    // 도전과제: `bedazzle`이라는 함수를 작성하세요. 이 함수는 String의 가변 참조를 받아들이고
    // 문자열 내용은 무시하고 문자열의 내용을 "sparkly"로 대체합니다. 그런 다음 아래 코드를 주석 해제하세요.
    //
    // 힌트: 새로운 값을 할당하기 위해 가변 참조를 참조해제해야 합니다.
    //
    let mut material = "mud".to_string();
    println!("이 재료는 단지 `{}`입니다.", material);
    bedazzle(&mut material);
    println!("와우! 이제 재료는 `{}`입니다!", material);
}

fn bedazzle(p0: &mut String) {
    *p0 = String::from("sparkly");
}

fn eat(p0: String) -> bool {
    if p0.starts_with("b") && p0.contains("a") {
        true
    } else {
        false
    }
}

fn change(p0: &mut String) {
    if !p0.ends_with('s') {
        *p0 += "s";
    }
}

fn test() {
    #[derive(Debug, Clone)]
    struct MyStruct {
        x: i32,
        y: i32,
        names: Vec<String>
    }

    fn main() {
        let original = MyStruct { x: 10, y: 20, names: vec![String::from("kim"), String::from("kang")] };
        let copied = original; // 이동 발생
        println!("{:?}", original); // 컴파일 에러. 복사 이후에도 원본 변수를 사용할 수 없음
    }


}

fn inspect(p0: &String) {
    if p0.ends_with('s') {
        println!("복수")
    } else {
        println!("단수")
    }
}
