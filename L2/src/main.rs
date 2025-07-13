use::std::{io, io::Write};

fn main() { 
    let mut checker  = "off"; //var for checking if ac or heater is on

    loop {
    //instructs the user on how to stop le program
    println!("If you wish to terminate type 'stop'");
    
    //prompts the user to enter the current room temperature
    print!("Please enter the current temperature: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let input = input.trim();
    
    if input == "stop"  {
        println!("Terminating the program");
        break;
    }

    let temp:i32 = input.parse().unwrap(); //converts input to i32

    // temp checking system
    if temp > 24 && checker == "off" {
        println!("Turning on AC");
        checker = "AC ON";
    } else if temp < 18 && checker == "off" {
        println!("Turning on Heater");
        checker = "Heater ON";
    } else if checker != "off" && temp > 24 { //checks if any system is on
        println!("Leaving AC on");
    } else if checker != "off" && temp < 18 {
        println!("Leaving Heater on");
    } else { 
        if checker == "AC ON" { 
            println!("Temperature is at normal levels, turning off AC");
            checker  = "off";
        } else if checker == "Heater ON" {
            println!("Temperature is at normal levels, turning off Heater");
            checker  = "off";
        } else {
            println!("Temperature is at normal levels, enjoy :D"); 
        }
    }

    //pretends the user has to wait for 5 mins
    println!("Please wait 5 minutes for the next check");

    }
    
}