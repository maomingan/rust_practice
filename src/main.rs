mod variables;
mod datatype;
mod control;
mod func;
mod refstring;
mod slice;
mod struct_demo;

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

    // 五、控制权、引用借用、slice
    // refstring::ref_test();
    // slice::test_err_slice();
    // slice::test_slice();

    // 六、结构体
    // struct_demo::test();
    // struct_demo::test2();
    struct_demo::test3();

}