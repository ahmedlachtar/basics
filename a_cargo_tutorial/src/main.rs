use std::io;

fn main() {
    let mut x:f32 = 4.8;
    println!("Number {}", x);
    {
        let x:f32 = x - 30.0;
        println!("Number {}", x);
    }
    x = x + 3.0;
    println!("Number {}", x);
    const SECONDS_IN_MINUTE: u32 = 60;
    println!("constant {}", SECONDS_IN_MINUTE);
    let mut arr:[i32; 5] = [1, 2, 3, 4, 5];
    println!("array {}", arr[0]);
    arr[0] = 20;
    println!("array {}", arr[0]);

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    println!("You typed: {}", input);
}

