fn main() {
    let x = 3;
    if x < 5 {
        println!("条件为真");
    } else {
        println!("条件为假");
    }

    match x < 5 {
        true => println!("利用match 条件为真"),
        false => println!("利用match 条件为假"),
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);
}
