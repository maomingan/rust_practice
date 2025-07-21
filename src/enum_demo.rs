#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("call : {:#?}", self);
    }
}

fn main() {

    let msg = Message::Move { x: 1, y: 2 };
    msg.call();
    println!("msg: {:?}", msg);

    let msg2 = Message::Write(String::from("hello"));
    msg2.call();
    println!("msg2: {:?}", msg2);

    let msg3 = Message::ChangeColor(255, 255, 255);
    msg3.call();
    println!("msg3: {:?}", msg3);

    let msg4 = Message::Quit;
    msg4.call();
    println!("msg4: {:?}", msg4);
    
}