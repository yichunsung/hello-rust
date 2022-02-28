fn main() {
    println!("Hello, world!");

    let mut x: u64 = 5;
    println!("x 的數值為：{}", x);
    x = 6;
    println!("x 的數值為：{}", x);

    let hello_world: &str = "hello world";
    println!("{}", hello_world);
    hello(x);
}

fn hello(arg: u64) {
  println!("{:?}", arg);
}