fn main() {
    hello_world();
    data_type();
    tuples_and_array();
    hello_shadowing();
    if_else_function();
    loop_function();
    loop_result();
    hello_while();
    for_loop();
    string_data();

    let hello_arg: i32 = 22;

    take_copy(hello_arg);
    println!("main: {:?}", hello_arg);

    let hello_string = String::from("你好唷～～～");

    // take_clone(hello_string);
    let new_string = return_string(hello_string);

    println!("main: {:?}", new_string);

    let test_string = String::from("YOYOYO!");

    let (test_string_2, leng) = cal_len_with_taup(test_string);
    println!("string: {}, data: {}", test_string_2, leng);

    let str3 = String::from("你好我好大家好");
    let str3_leng = cal_leng_with_reference(&str3);
    println!("string: {}, data: {}", str3, str3_leng);

    let mut string_4 = String::from("大阪桐蔭高等学校吹奏楽部");

    use_mut_in_reference(&mut string_4);

    println!("{}", string_4);

}

fn hello_world() {
    println!("Hello, world!");
    let mut x: u64 = 5;
    println!("x 的數值為：{}", x);
    x = 6;
    println!("x 的數值為：{}", x);

    let hello_world: &str = "hello world";
    println!("{}", hello_world);
    let data = hello(x);
    println!("{}", data);
}

// 可變引用
fn use_mut_in_reference(message: &mut String) {
    message.push_str("yo~");
}

fn cal_leng_with_reference(string_data: &String) -> usize {
    string_data.len()
}

// use taup
fn cal_len_with_taup(string_data: String) -> (String, usize) {
    let len = string_data.len();
    (string_data, len)
}

fn take_copy(num: i32) {
    println!("take_copy: {}", num);
}

/*fn take_clone(string_data: String) {
     println!("take_clone: {}", string_data);
}*/

fn return_string(mut string_data: String) -> String {
    string_data.push_str("Yohi");
    return string_data;
}

fn string_data() {
    let mut string_data_type = String::from("你好");
    string_data_type.push_str(" 我好");
    let string_2 = string_data_type.clone();
    println!("{}", string_data_type);
    println!("{}", string_2);
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
        println!("hello_shadowing first {:?}", x);
        x + 3
    };
    println!("hello_shadowing second {:?}", y);
    println!("X: {:?}", x);
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

