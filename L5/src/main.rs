use::std::{io, io::Write};

fn pascal(x: u32, y: u32) -> u32{
    if y == 0 || y == x {
        1
    } else {
        pascal(x - 1, y - 1) + pascal(x - 1, y)
    }
}


fn main() {
    loop {
        let mut input = String::new();
        print!("Enter your number between 1-9: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let n:u32 = input.trim().parse().unwrap();

        if n > 9 {
            print!("Your number was too high");
            break;
        }

        for i in 0..n {
            for _ in 0..(n - i) {
                print!("  ");
            }
            for j in 0..=i {
                print!("{:4}", pascal(i, j));
            }
            println!()
            }
            break;
        }
    
    
}