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
    // 3. mut只是可以修改变量，而不能修改类型
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

    // 使用下划线开头忽略未使用的变量
    // result: ^ help: 如果 y 故意不被使用，请添加一个下划线前缀: `_y`
    let _x = 5;
    let y = 10;

    // 变量解构

    let (a, mut b): (bool, bool) = (true, false);
    println!("a = {:?}, b = {:?}", a, b);
    // result: a = true, b = false

    b = true;
    assert_eq!(a, b);
    println!("a = {:?}, b = {:?}", a, b);
    // result: a = true, b = true

    // 解构式赋值

    let (a, b, c, d, e);
    (a, b) = (1, 2);
    // _ 代表匹配一个值，但是我们不关心具体的值是什么，因此没有使用一个变量名而是使用了 _
    [c, .., d, _] = [1, 2, 3, 4, 5];
    Struct { e, .. } = Struct { e: 5 };

    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);
    println!("a = {:?}, b = {:?}, c = {:?}, d = {:?}, e = {:?}", a, b, c, d, e);
    // result: a = 1, b = 2, c = 1, d = 4, e = 5


    // 变量遮蔽(shadowing)

    let x = 5;
    // 在main函数的作用域内对之前的x进行遮蔽
    let x = x + 1;
    {
        // 在当前的花括号作用域内，对之前的x进行遮蔽
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
    // result: The value of x in the inner scope is: 12
    // result: The value of x is: 6

    // 由于Rust对变量类型的要求严格，若这里使用let mut spaces = "   "，编译会报错
    // mismatched types, expected `&str`, found `usize`
    let spaces = "   ";
    let spaces = spaces.len();
    println!("{}", spaces);
    // result: 3

}


struct Struct {
    e: i32
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