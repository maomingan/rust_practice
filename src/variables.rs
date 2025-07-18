pub fn variables() {
    let mut x = 5;
    println!("the x is {}", x);
    x = 6;
    println!("the x is {}", x);

    const ORIGIN_X: i32 = 0;
    x = ORIGIN_X;
    println!("the x is {}", x);

    let x = 10;
    {
        let x = x * 2;
        println!("the x in inner scope is : {}", x);
    }
    println!("the x is {}", x);
}