// 1. `Bite`라는 트레이트를 정의합니다.
//
// 하나의 필수 메서드인 `fn bite(self: &mut Self)`를 정의합니다. 우리는 뭔가를 물어뜯을 때 이 메서드를 호출할 것입니다.
// 이 트레이트를 정의한 후에는 `cargo run`을 실행했을 때 오류 없이 실행되어야 합니다.
//
trait Bite {
    fn bite(self: &mut Self);
}


// 2. 이제 "Grapes"라는 이름의 구조체를 만들어서 남은 포도의 수를 추적하는 필드를 만듭니다.
// 힌트가 필요하다면 파일 아래의 Carrot을 살펴보세요 (그러나 다른 필드를 사용하는 것이 좋을 것입니다).
//
// #[derive(Debug)] // 구조체 정의 바로 앞에 이 라인을 포함하세요
#[derive(Debug)]
struct Grapes {
    amount_left: i32,
}

// 3. Grapes에 대한 Bite를 구현합니다. 포도를 물면 남은 포도의 수에서 1을 뺍니다.
// 힌트가 필요하다면 파일 아래의 Carrot을 살펴보세요.
//
// impl Bite for...

impl Bite for Grapes {
    fn bite(self: &mut Self) {
        self.amount_left -= 1;
    }
}


fn main() {
    // 1번을 완료하면 이 부분은 작동해야 합니다.
    let mut carrot = Carrot { percent_left: 100.0 };
    carrot.bite();
    println!("한 입 베어 먹었어: {:?}", carrot);

    // 4. 아래 코드를 주석 해제하고 그레이프(struct Grapes)를 정의한 방식에 맞게 조정하세요.
    //
    let mut grapes = Grapes { amount_left: 100 };
    grapes.bite();
    println!("포도 한 알 먹었어: {:?}", grapes);

    // 챌린지: 아래 코드를 주석 해제하세요. `Bite`를 구현한 모든 타입의 가변 참조를 가져와서
    // `.bite()`를 여러 번 호출합니다.
    // 힌트: 함수 이름과 괄호를 열기 전에 제네릭 타입을 정의하세요:
    //       fn function_name<T: Bite>(...)
    //
    bunny_nibbles(&mut carrot);
    println!("토끼가 조금씩 먹어갔어: {:?}", carrot);

    bunny_nibbles(&mut grapes);
    println!("토끼가 조금씩 먹어갔어: {:?}", grapes);
}

fn bunny_nibbles<T: Bite>(p0: &mut T) {
    p0.bite();
}

#[derive(Debug)] // 이것은 디버깅 포맷 문자열 "{:?}"을 사용할 수 있게 합니다.
struct Carrot {
    percent_left: f32,
}

impl Bite for Carrot {
    fn bite(self: &mut Self) {
        // 남은 당근의 20%를 먹습니다. 모두 먹는 데에는 시간이 걸릴 수 있습니다...
        self.percent_left *= 0.8;
    }
}
