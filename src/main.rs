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

    println!("----------------------");

    //                       2.1  变量绑定与解构

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

    println!("----------------------");

    //                            2.2 基本类型


    // let guess = "42".parse().expect("Not a number!");
    // compiler cannot infer the type of `guess`
    // It should be let guess: i32 = "42"... or let guess = "42".parse::<i32>()...



    //      Integers Types
    // integer:   i8, i16, i32, i64, i128, isize
    // unsigned:  u8, u16, u32, u64, u128, usize
    //
    // decimalism:     98_222
    // hexadecimal:    0xff
    // octal:          0o77
    // binary:         0b1111_0000
    // byte(u8 only):  b'A'

    //  整型溢出
    // 假设有一个 u8 ，它可以存放从 0 到 255 的值。那么当你将其修改为范围之外的值，比如 256，则会发生整型溢出。
    // 在当使用 --release 参数进行 release 模式构建时，Rust 不检测溢出。
    // 相反，当检测到整型溢出时，Rust 会按照补码循环溢出（two’s complement wrapping）的规则处理。
    // 大于该类型最大值的数值会被补码转换成该类型能够支持的对应数字的最小值。
    // 例如，如果 u8 类型的最大值是 255，那么 256 会被转换成 0, 257 会被转换成 1，依此类推。
    // 处理溢出方法：
    // 使用 wrapping_* 方法在所有模式下都按照补码循环溢出规则处理，例如 wrapping_add
    // 如果使用 checked_* 方法时发生溢出，则返回 None 值
    // 使用 overflowing_* 方法返回该值和一个指示是否存在溢出的布尔值
    // 使用 saturating_* 方法，可以限定计算后的结果不超过目标类型的最大值或低于最小值，例如:\
    // assert_eq!(100u8.saturating_add(1), 101);
    // assert_eq!(u8::MAX.saturating_add(127), u8::MAX);


    // warpping_* 方法
    let a : u8 = 255;
    let b = a.wrapping_add(20);
    println!("{}", b);  // result: 19


    // 浮点类型
    let _x = 2.0; // f64
    let _y: f32 = 3.0; // f32

    // 断言0.1 + 0.2与0.3相等
    // assert!(0.1 + 0.2 == 0.3);
    // 因为二进制精度问题，导致了 0.1 + 0.2 并不严格等于 0.3，它们可能在小数点 N 位后存在误差。

    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("   0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("         0.3: {:x}", (abc.2).to_bits());
    println!();

    println!("xyz (f64)");
    println!("   0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!("         0.3: {:x}", (xyz.2).to_bits());
    println!();

    //assert!(abc.0 + abc.1 == abc.2);
    //assert!(xyz.0 + xyz.1 == xyz.2);

    // result:
    // abc (f32)
    //    0.1 + 0.2: 3e99999a
    //          0.3: 3e99999a
    //
    // xyz (f64)
    //    0.1 + 0.2: 3fd3333333333334
    //          0.3: 3fd3333333333333


    // NaN
    // let x = (-42.0_f32).sqrt();
    // assert_eq!(x, x);
    // 对于数学上未定义的结果，例如对负数取平方根 -42.1.sqrt()
    // 会产生一个特殊的结果：Rust 的浮点数类型使用 NaN (not a number)来处理这些情况。
    // 出于防御性编程的考虑，可以使用 is_nan() 等方法，可以用来判断一个数值是否是 NaN
    let x = (-42.0_f32).sqrt();
    if x.is_nan() {
        println!("未定义的数学行为")
    }

    // 数字运算
    // 加法
    let sum = 5 + 10;
    // 减法
    let difference = 95.5 - 4.3;
    // 乘法
    let product = 4 * 30;
    // 除法
    let quotient = 56.7 / 32.2;
    // 求余
    let remainder = 43 % 5;
    println!("sum = {}, difference = {}, product = {}, quotient = {}, remainder = {}", sum, difference, product, quotient, remainder);

    // 编译器会进行自动推导，给予twenty i32的类型
    let twenty = 20;
    //类型标注
    let twenty_one: i32 = 21;
    // 通过类型后缀的方式进行类型标注：22是i32类型
    let twenty_two = 22i32;

    // 只有同类型才能进行运算
    let addition = twenty + twenty_one + twenty_two;
    println!("{} + {} + {} = {}", twenty, twenty_one, twenty_two, addition);

    // 对于较长的数字，可以用_进行分割，提高可读性
    let one_million: i64 = 1_000_000;
    println!("{}", one_million.pow(2));

    // 定义一个f32数组，其中42.0会自动被推导为f32类型
    let forty_twos = [42.0, 42f32, 42.0_f32,];
    // 打印数组中第一个值，并控制小数位为2位
    println!("{:.2}", forty_twos[0]);

    println!("----------------------");

    // Rust 位运算法则
    //
    // & 位与 (BIT AND), 相同位置均为1时则为1，否则为0
    // | 位或 (BIT OR), 相同位置只要有1时则为1，否则为0
    // ^ 异或 (BIT XOR), 相同位置不相同则为1，相同则为0
    // ! 位非 (BIT NOT), 把位中的0和1相互取反，即0置为1，1置为0
    // << 左移 (BIT LEFT SHIFT), 所有位向左移动指定位数，右位补0
    // >> 右移 (BIT RIGHT SHIFT), 所有位向右移动指定位数，带符号移动（正数补0，负数补1）

    // 二进制为00000010
    let a:i32 = 2;
    // 二进制为00000011
    let b:i32 = 3;

    println!("(a & b) value is {}", a & b);
    println!("(a | b) value is {}", a | b);
    println!("(a ^ b) value is {}", a ^ b);
    println!("(!b) value is {} ", !b);
    println!("(a << b) value is {}", a << b);
    println!("(a >> b) value is {}", a >> b);

    let mut a = a;
    a <<= b;
    println!("(a << b) value is {}", a);

    println!("----------------------");

    // 序列 （Range）
    // Rust 提供了一个非常简洁的方式，用来生成连续的数值
    //例如 1..5，生成从 1 到 4 的连续数字，不包含 5 ；1..=5，生成从 1 到 5 的连续数字，包含 5
    for i in 1..=5 {
        println!("{}", i);
    }
    // result: 1 2 3 4 5

    for i in 'a'..='z' {
        println!("{}", i);
    }
    // result: a b c d e f g h i j k l m n o p q r s t u v w x y z

    println!("----------------------");

    // 使用 As 完成类型转换
    // Rust 中可以使用 As 来完成一个类型到另一个类型的转换，其最常用于将原始类型转换为其他原始类型，
    //但是它也可以完成诸如将指针转换为地址、地址转换为指针以及将指针转换为其他指针等功能

    
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