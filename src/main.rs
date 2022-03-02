fn main() {
    println!("Hello, world!");

    let mut x: u64 = 5;
    println!("x 的數值為：{}", x);
    x = 6;
    println!("x 的數值為：{}", x);

    let hello_world: &str = "hello world";
    println!("{}", hello_world);
    let data = hello(x);
    println!("{}", data);
    data_type();
    tuples_and_array();
    hello_shadowing();
    if_else_function();
    loop_function();
    loop_result();
    hello_while();
    for_loop();
}

fn for_loop() {
    const ARRAY_DATA: [i32; 5] = [55, 66, 22, 231, 123];
    for ele in ARRAY_DATA {
        println!("{}", ele);
    }
}

fn hello_while() {
    let mut n_data: u32 = 0;
    while n_data <= 10 {
        n_data += 1;
        println!("{}", n_data);
    }
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("數值為：{}", a[index]);

        index += 1;
    }
}

fn loop_result() {
    let mut hello: u32 = 0;
    let result = loop {
        hello += 2;
        if hello >= 20 {
            break hello;
        }
    };
    println!("result: {}", result);
}

fn loop_function() {
    let mut number_data: u32 = 0;
     loop {
        if number_data >= 10 {
            break;
        }

        println!("Hello: {}", number_data);
        number_data += 1;

    }
}

fn if_else_function() {
    let number: u32 = 2;
    if number > 30 {
        println!("{}", "hello");
    } else {
        println!("{:?}", "nonononono");
    }

    if number == 10 {
        println!("{}", "toto");
    } else if number == 2 {
        println!("{}", number == 2);
    }

    let data_menu = if number == 2 { 5 } else { 10 };

    println!("{}", data_menu);
}

// 遮蔽
fn hello_shadowing() {
    let x: i32 = 33;
    let y = {
        let x = x + 2;
        println!("{:?}", x);
        x + 3
    };
    println!("{:?}", y);
}

fn hello(arg: u64) -> u64 {
    return arg + 2;
}

fn data_type() {
    let mut integer: u64 = 30;
    integer = integer + 3;
    println!("{:?}", integer);
    const SIGN_DATA: i64 = -22;
    println!("{:?}", SIGN_DATA);

    let hello_float: f32 = 20.22;
    println!("{:?}", hello_float);

    let hello_boolean: bool = true;
    println!("{:?}", hello_boolean);

    // 字元
    let char_data: char = 'y';
    println!("{:?}", char_data);
}

fn tuples_and_array() {

    // tuples
    let mut tup: (i32, i32, bool) = (500, 500, false);
    println!("{:?}", tup);

    tup.1 = 5566;

    let (a, b, c) = tup;
    println!("{:?} {:?} {:?}", a, b, c);

    let first_data = tup.2;
    println!("{:?}", first_data);

    // Array
    let new_array = [true, false, true];
    println!("{:?}", new_array);

    let mut hello_array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", hello_array);
    hello_array = [100, 200, 330, 444, 5566];
    println!("{:?}", hello_array);

    let data = [true; 5];
    println!("{:?}", data);
    println!("洗洗睡：{}", hello_array[4]);
}

