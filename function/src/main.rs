fn main() {
    let x = plus_one(32);
    println!("The function exec return is: {}", x);

    let y = 59;
    println!("x: {}, y: {}, x + y = {}", x, y, add(x, y));
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}
