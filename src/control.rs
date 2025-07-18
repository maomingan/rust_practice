pub fn control_if(number: i32){
    if number < 0 {
        println!("number {} is less than 0!", number);
    } else if number == 0 {
        println!("number {} is 0!", number);
    } else {
        println!("number {} is large than 0!", number);
    }
}

pub fn control_if_let(b: bool) {
    let result = if b {"yes"} else {"no"};
    println!("the result is {}", result);
}

pub fn control_loop () {
    let mut sum = 0;
    let mut number = 0;
    loop {
        number += 1;
        sum += &number;
        if number == 100 { break; }
    }
    println!("sum is : {}", sum);
}

pub fn control_while () {
    let mut sum = 0;
    let mut number = 0;
    while number < 100 {
        number += 1;
        sum += &number;
    }
    println!("sum is : {}", sum);
}

pub fn control_for () {
    let mut sum = 0;
    for number in 1..101 {
        sum += number;
    }
    println!("sum is : {}", sum);
}