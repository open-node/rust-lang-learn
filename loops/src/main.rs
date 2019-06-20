fn main() {
    let mut times = 0;
    loop {
        println!("loop times: {}", times);
        times += 1;
        if times > 1000 {
            break;
        }
    }

    let result = loop {
        times -= 1;
        if times < 1 {
            break 999;
        }
    };

    println!("result is: {}", result);

    while times < 100000000 {
        times += 1
    }

    println!("times is: {}", times);

    // 利用 for 遍历数组
    println!("利用for in 遍历数组");
    let a = [1, 2, 3, 4, 5];
    for item in a.iter() {
        println!("The item value is: {}", item);
    }

    // 第二种利用 while 编辑数组的方式
    println!("利用 while 递增 index 遍历数组");
    let mut index = 0;
    while index < a.len() {
        println!("The item value is: {}", a[index]);
        index += 1;
    }
}
