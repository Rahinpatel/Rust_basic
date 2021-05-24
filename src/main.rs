fn main() {
    println!("Hello, World!");
    add()
}

fn add() {
    let mut x = 24;
    println!("the value of x is {}", x);
    x = 60;
    println!("the value of x is {}", x);
    data()
}

fn data() {
    let n: u64 = 45;
    if n < 55 {
        println!("{}", n);
        println!("test data");
    } else {
        println!("test2")
    }
    data_loop()
}

fn data_loop() {
    let mut n = 0;
    loop {
        n += 1;
        if n == 7 {
            continue;
        }
        if n > 10 {
            break;
        }
        println!("the value of n is {}", n);
    }
    while_loop()
}

fn while_loop() {
    let mut n = 1;
    while n <= 50 {
        if n % 2 == 0 {
            println!("n is {}", n);
        }
        n += 1;
    }
    for_loop()
}

fn for_loop() {
    let number = 30..51;
    let animals = vec!["a", "b"];
    for i in number {
        println!("The number is {}", i)
    }
    for a in animals.iter() {
        println!("The number is {}", a)
    }
    rust_switch_case()
}


enum Direction {
    Up,
    Down,
}

fn rust_switch_case() {
    let enum_data: Direction = Direction::Up;

    match enum_data {
        Direction::Up => println!("hi"),
        Direction::Down => println!("hi"),
    }
}
//
