use::std::{io, io::Write};

fn ctf(c:f64) -> f64 {
    (c * 9.0/5.0) + 32.0
}

fn ftc(f:f64) -> f64 {
    (f - 32.0) * 5.0/9.0
}

fn show_temp(label: &str, value: &f64) {
    println!("{} temperature: {:.2} degrees {}", label, value, label)
}

fn adjust_temp(value: &f64, delta: f64) {
    let mut value = *value;
    value += delta;
    println!("Adjusted value: {}", value);

}


fn main() {
    println!("1. Celcius -> Fahrenheit");
    println!("2. Fahrenheit -> Celcius");

    let mut choice = String::new();
    print!("Enter your choice: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut choice).expect("Invalid input");
    
    match choice.trim() {
        "1" => {
            let mut cel = String::new();
            print!("Enter celius temp: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut cel).expect("Invalid input");
            let mut cel:f64 = cel.trim().parse().expect("Invalid temp value");
            cel = ctf(cel);
            let fah = "Fahrenheit";
            show_temp(&fah, &cel);
            adjust_temp(&cel, 0.5);

        }
        "2" => {
            let mut fah = String::new();
            print!("Enter fahrenheit temp: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut fah).expect("Invalid input");
            let mut fah:f64 = fah.trim().parse().expect("Invalid temp value");
            fah = ftc(fah);
            let cel = "Celcius";
            show_temp(&cel, &fah);
            adjust_temp(&fah, 0.5);
        }
        _ => {
            println!("Invalid input");
        }
    }

}