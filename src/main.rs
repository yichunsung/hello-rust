struct User {
    id: u32,
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(name: String, email: String) -> User {
    User {
        id: 1,
        active: true,
        email: email,
        username: name,
        sign_in_count: 1,
    }
}

fn area(width: u32, height: u32) -> u32 {
    return width * height;
}

fn area2(dimensions: (u32, u32)) -> u32 {
    return dimensions.0 * dimensions.1;
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn area_rectangle(rectangle: &Rectangle) -> u32 {
    return rectangle.width * rectangle.height;
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// Enum
enum IpAddressKind {
    ip4,
    ip6
}

struct IpAddress {
    kind: IpAddressKind,
    address: String,
}

#[derive(Debug)]
enum IpAdr {
    v4(String),
    v6(String)
}

fn route(ip_kind: IpAddressKind) {
    println!("------");
    let home = IpAddress {
        kind: IpAddressKind::ip4,
        address: String::from("199.22.235.54"),
    };

    let loopback = IpAddress {
        kind: IpAddressKind::ip6,
        address: String::from("::1"),
    };

    let new_home = IpAdr::v4(String::from("212.22.66.733"));
    let loopback_adr = IpAdr::v6(String::from("::1"));
    println!("{:?}", new_home);
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct QuitMessage;

struct MoveMessage(Message);

fn main() {
    // 元祖
    let tup: (i32, f64, u8) = (500, 3.1, 4);
    println!("{}", tup.0);
    // 陣列
    let array: [i32; 4] = [2, 33, 2, -2];
    println!("{}", array[0]);

    let mut user1: User = User {
        id: 1,
        active: true,
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        sign_in_count: 1,
    };
    println!("{}", user1.username);
    user1.username = String::from("Hello");
    println!("{}", user1.username);

    let user2: User = build_user(String::from("My Name"),  String::from("abc@email.com"));
    println!("{}", user2.username);

    println!("-----");

    let height: u32 = 20;
    let width: u32 = 6;
    let area_data = area(height, width);
    println!("area: {}", area_data);

    println!("------");

    let area_tup: (u32, u32) = (500, 32);
    let area2_data = area2(area_tup);

    println!("area with tup, result: {}", area2_data);

    
    let react1: Rectangle = Rectangle {
        width: 33,
        height: 99
    };

    let area_react: u32 = area_rectangle(&react1);

    println!("area_react: {}", area_react);

    println!("rect1 is {:?}", react1);

    println!(
        "長方形的面積為 {} 平方像素。",
        react1.area()
    );

    // Enum
    let data_v4: IpAddressKind = IpAddressKind::ip4;
    let data_v5: IpAddressKind = IpAddressKind::ip6;

    route(data_v4);
    route(data_v5);
    /* 
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

    let hello_string: String = String::from("你好唷～～～");

    // take_clone(hello_string);
    let new_string: String = return_string(hello_string);

    println!("main: {:?}", new_string);

    let test_string: String = String::from("YOYOYO!");

    let (test_string_2, leng) = cal_len_with_taup(test_string);
    println!("string: {}, data: {}", test_string_2, leng);

    let str3: String = String::from("你好我好大家好");
    let str3_leng: usize = cal_leng_with_reference(&str3);
    println!("string: {}, data: {}", str3, str3_leng);

    let mut string_4: String = String::from("大阪桐蔭高等学校吹奏楽部");

    use_mut_in_reference(&mut string_4);

    println!("{}", string_4);
*/
}

/*
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

*/
