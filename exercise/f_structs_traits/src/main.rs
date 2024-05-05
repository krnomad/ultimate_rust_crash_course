trait Run {
    fn run(&self) {
        println!("I'm running");
    }
}

struct Robot {}

impl Robot {
    pub fn new() -> Self {
        Self {}
    }
}

impl Run for Robot {
    fn run(&self) {
        println!("로봇 뛴다");
    }
}

struct Human {}

impl Human {
    pub fn new() -> Self {
        Self {}
    }
}

impl Run for Human {}

fn run<T: Run>(object: T) {
    object.run();
}

fn main() {
    let robot = Robot::new();
    let human = Human::new();

    run(robot);
    run(human);
}
