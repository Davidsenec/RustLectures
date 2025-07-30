use::std::{io, io::Write};

fn pascal(x: u32, y: u32) -> u32{
    if y == 0 || y == x {
        1
    } else {
        pascal(x - 1, y - 1) + pascal(x - 1, y)
    }
}


fn main() {
    let mut input = String::new();
    print!("Enter your number: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let n:u32 = input.trim().parse().unwrap();

    for i in 0..n {
        for _ in 0..(n - i - 1) {
            print!(" ");
        }
        for j in 0..=i {
            print!("{:2}", pascal(i, j));
        }
        println!()
    }
}