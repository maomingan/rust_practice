fn main() {
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("{:?}", v);

    // 传递了一个不存在的索引时，会panic
    let first = &v[0];
    println!("first: {}", first);

    // 这种方式不会panic，返回的是一个Option<&T>
    let second = v.get(1);
    println!("second: {:?}", second);

    match second {
        Some(second) => println!("optinal second: {}", second),
        None => println!("optinal second is none"),
    }
    
    // 获取second的值
    let second = second.unwrap();
    println!("real second: {}", second);

    // 遍历vector
    let v2 = vec![1, 2, 3];
    for i in &v2 {
        println!("i: {}", i);
    }

    // 遍历vector并修改
    let mut v3 = vec![1, 2, 3];
    for i in &mut v3 {
        *i += 50;
        println!("i: {}", i);
    }
    
    // 使用枚举来存储多种类型
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for cell in &row {
        match cell {
            SpreadsheetCell::Int(value) => println!("Int: {}", value),
            SpreadsheetCell::Float(value) => println!("Float: {}", value),
            SpreadsheetCell::Text(value) => println!("Text: {}", value),
        }
    }
   
}