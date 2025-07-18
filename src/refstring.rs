pub fn ref_test () {
    // 字面量字符串，不可变
    let mut s = "a";
    s = "b";
    let s2 = s;
    println!("s is {}", s);

    // 真正的可变字符串，在堆中申请内存，作用域结束之后所有者ss会释放
    let ss = String::from("aa");
    // let ss2 = ss;
    // 数据深拷贝
    let ss2 = ss.clone();
    println!("ss is {}", ss);
    test_string(ss);
    // 这里ss的作用域因为调用完函数后已经被drop，所以再次使用ss的时候内存没东西了，会编译报错
    // println!("{}", ss);
}

fn test_string(str: String){
    println!("test_string : {}", str);
}