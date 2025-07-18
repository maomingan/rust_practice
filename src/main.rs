mod variables;
mod datatype;
mod control;
mod func;

use variables::variables;
use datatype::datatype;

fn main() {
    println!("Hello, world!");
    println!("-------------------------");
    // 一、变量
    // variables();
    // 二、数据类型
    // datatype();
    // 三、函数
    // func::in_params(2, 5);
    // println!("获取返回值：{}", func::return_result(1,1));
    // 四、控制器
    // control::control_if(-1);
    // control::control_if(0);
    // control::control_if(1);
    // control::control_if_let(true);
    // control::control_if_let(false);
    // control::control_loop();
    // control::control_while();
    // control::control_for();

    let mut s = "a";
    s = "b";
    let s2 = s;
    println!("s is {}", s);
    let ss = String::from("aa");
    // let ss2 = ss;
    let ss2 = ss.clone();
    println!("ss is {}", ss);
    test_string(ss);
    println!("{}", ss);

}

fn test_string(str: String){
    println!("test_string : {}", str);
}