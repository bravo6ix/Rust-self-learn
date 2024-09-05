fn main() {
    // println!("Hello, world!");

    //greet_world();

    // 使用let来声明变量，进行绑定，a是不可变的
    // 此处没有指定a的类型，编译器会默认根据a的值为a推断类型：i32，有符号32位整数
    // 语句的末尾必须以分号结尾
    let a = 10;

    // 主动指定b的类型为i32
    let b: i32 = 20;

    // 这里有两点值得注意：
    // 1. 可以在数值中带上类型:30i32表示数值是30，类型是i32
    // 2. c是可变的，mut是mutable的缩写
    #[allow(unused_mut)]  //This code is for ignoring the warning
    let mut c =30i32;

    // 还能在数值和类型中间添加一个下划线，让可读性更好
    let d = 30_i32;

    // 跟其它语言一样，可以使用一个函数的返回值来作为另一个函数的参数
    let e = add(add(a, b), add(c, d));

    // println!是宏调用，看起来像是函数但是它返回的是宏定义的代码块
    // 该函数将指定的格式化字符串输出到标准输出中(控制台)
    // {}是占位符，在具体执行过程中，会把e的值代入进来
    println!("( a + b ) + ( c + d ) = {}", e);
}

// fn greet_world() {
//     let southern_germany = "Grüß Gott!";
//     let chinese = "世界!";
//     let spanish = "Hola Mundo";
//     let english = "Hello World";
//     let regions = [southern_germany, chinese, spanish, english];
//     for region in regions.iter() {
//         println!("{}", region);
//     }
// }

fn add(i: i32, j: i32) -> i32 {
    i + j
}