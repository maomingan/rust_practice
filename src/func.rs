pub fn in_params(x: i32, y: i32) {
    let result = x + y;
    println!("{} + {} = {}", x, y, result);
}

pub fn return_result(x: i32, y: i32) -> i32 {
    let result = x + y;
    result
}