use std::io;

pub fn datatype() {
    // 基本数据类型
    let number_i = 23; // 有符号整形，可以表示负数
    let number_u: u32 = 23; // 无符号整形，只能表示正数
    let number_f = 0.2; // 双精度浮点
    let number_f32: f32 = 0.2;  // 单精度浮点
    let b = false;  // 布尔
    let c = 'Z';    // char，四字节，单引号
    let s = "Z";    // 字符串

    // 复合数据类型
    // 元祖
    let tup = (650, 0.1, "gump");
    println!("{}", tup.0);
    println!("{}", tup.1);
    println!("{}", tup.2);
    let (t1, t2) = (1, 0.58);
    println!("{}", t1);
    println!("{}", t2);

    // 数组
    let arr = [1,2,3,4,5];
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input.");
    let index: usize = input.trim().parse().expect("Failed to parse input to number.");
    println!("input index {} then get value : {}", index, arr[index]);
}