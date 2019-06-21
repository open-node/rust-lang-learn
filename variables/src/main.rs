const MAX_POINTS: u32 = 100_000_000;
const MAX_POINTS_2: u32 = 1000 * 1000;

fn main() {
    let x: f64 = 0.0;
    println!("This value of x is: {}", x);
    let x: f64 = x - 1.0;
    let y = x / 2.5;
    println!("This value of x is: {}", x);
    println!("This value of y is: {}", y);
    println!("MAX_POINTS value is: {}", MAX_POINTS);
    println!("MAX_POINTS_2 value is: {}", MAX_POINTS_2);

    let tup: (i32, f64, u8) = (500, 2.3, 31);
    let (x, y, z) = tup;
    println!("x = {}, y = {}, z = {}", x, y, z);
    println!("tup.0 = {}, tup.1 = {}, tup.2 = {}", tup.0, tup.1, tup.2);

    // array
    let array: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("Array first item is: {}", array[0]);
}
