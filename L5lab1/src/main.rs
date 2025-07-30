use::std::{io, io::Write};

fn ctf(c:f64) -> f64 {
    (c * 9.0/5.0) + 32.0
}

fn ftc(f:f64) -> f64 {
    (f - 32.0) * 5.0/9.0
}

fn main() {
    println!("1. Celcius -> Fahrenheit");
    println!("2. Fahrenheit -> Celcius");

    let mut choice = String::new();
    print!("Enter your choice: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line( &mut choice).expect("Invalid input");
    
    match choice.trim() {
        "1" => {
            let mut cel = String::new();
            print!("Enter celius temp: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut cel).expect("Invalid input");
            let cel:f64 = cel.trim().parse().expect("Invalid temp value");
            println!("{} degrees fahrenheit", ctf(cel));

        }
        "2" => {
            let mut fah = String::new();
            print!("Enter fahrenheit temp: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut fah).expect("Invalid input");
            let fah:f64 = fah.trim().parse().expect("Invalid temp value");
            println!("{} degrees celcius", ftc(fah));
        }
        _ => {
            println!("Invalid input");
        }
    }

}
