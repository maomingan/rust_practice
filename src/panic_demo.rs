use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    // 自己调用panic宏，程序会直接退出
    // self_call_panic();

    // 程序异常，调用了别的包的panic
    // arr_panic();

    // 获取当前目录下的文件，如果没有则可以捕获可预见的异常，并panic
    // try_get_file();

    // 没有文件的时候，创建文件
    // no_file_then_create_file1();
    // no_file_then_create_file2();

    // 使用unwrap获取文件
    // unwrap_test();

    // 使用expect获取文件
    // expect_test();

    // 错误传播冗余写法
    // read_username_from_file();

    // 传播错误的快捷方式：? 运算符，只能在返回 Result、Option 或者其它实现了 FromResidual 的类型的函数中使用 ? 运算符
    read_username_from_file2();

    // 使用已有crate的函数
    // read_username_from_file3();


}

fn self_call_panic() {
    panic!("self call panic");
}

fn arr_panic() {
    let v = vec![1, 2, 3];
    v[99];
}

fn try_get_file() -> File {
    let f = File::open("hello.txt");
    match f {
        Ok(file) => file,
        Err(e) => panic!("Problem opening the file: {:?}", e),
    }
}

fn no_file_then_create_file1() -> File {
    let f = File::open("hello.txt");
    match f {
        Ok(file) => file,
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            _ => panic!("Problem opening the file: {:?}", e),
        },
    }
}

fn no_file_then_create_file2() -> File {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == io::ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    return f;
}

fn unwrap_test() -> File {
    let f = File::open("hello.txt").unwrap();
    return f;
}

fn expect_test() -> File {
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
    return f;
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut file = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file3() -> Result<String, io::Error> {
    return fs::read_to_string("hello.txt");
}