// use std::env::consts;

fn modifyv(mut x: i32) {
    x += 10;
    println!("Inside func: {}", x);
}

fn modifyp(x: &mut i32) {
    *x += 10;
    println!("In func: {} {} {} {:p}", *x, x, &x, x as *const i32);
}

fn main() {
    let mut a = 20;
    modifyv(a);
    println!("In main: {}", a);
    modifyp(&mut a);
    println!("In main: {}", a);
    println!("{}", double(8));
    println!("{}", sum(2));
    say_hello("Alice");
    match safe_divide(10, 0) {
        Ok(result) => println!("Result: {}", result),
        Err(error) => println!("Error: {}", error)
    }
}

fn double(x:i32) -> i32 {
    x * 2
}

fn sum(n:u32) -> u32 {
    if n == 0 {
        1
    } else {
        n + sum(n - 1)
    }
}

fn say_hello(name: &str) {
    println!("Hello, {}!", name);

}

fn safe_divide(a:i32, b:i32) -> Result<i32, String> {
    if b == 0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}