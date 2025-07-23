use std::io::{self};

fn main() {
    let mut warehouse:Vec<(u32, String, u32)> = Vec::new();

    loop {
        println!("Warehouse Inventory Management:");
        println!("1. Add new product");
        println!("2. Update stock quantity");
        println!("3. Remove product");
        println!("4. List all products");
        println!("5. Exit");

        println!("Enter your choice: ");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");

        match choice.trim() {
        "1" => {
            println!("Enter new product ID: ");
            let mut newID = String::new();
            io::stdin().read_line(&mut newID).expect("Failed to read input");
            let newID:u32 = newID.trim().parse().expect("Invalid ID");

            let mut newName = String::new();
            println!("Enter new product name: ");
            io::stdin().read_line(&mut newName).expect("Failed to read input");
            let newName = newName.trim().to_string();
            
            let mut newQuantity = String::new();
            println!("Enter quantity: ");
            io::stdin().read_line(&mut newQuantity).expect("Failed to read input");
            let newQuantity:u32 = newQuantity.trim().parse().expect("Invalid quantity");

            warehouse.push((newID, newName, newQuantity));
            println!("Added new product");
        }

        "2" => {
            println!("Enter ID desired product to update quantity: ");
            let mut ID = String::new();
            io::stdin().read_line(&mut ID).expect("Failed to read input");
            let ID:u32 = ID.trim().parse().expect("Invalid ID");
            let mut i = 0;
            let mut id_exists = false;
            for (IDP, _, _) in &warehouse {
                if *IDP == ID {
                    let mut newQuantity = String::new();
                    println!("Enter quantity: ");
                    io::stdin().read_line(&mut newQuantity).expect("Failed to read input");
                    let newQuantity:u32 = newQuantity.trim().parse().expect("Invalid quantity");
                    id_exists = true;
                    warehouse[i].2 = newQuantity; 
                    println!("Removed item {ID}");
                    break;
                } else {
                    println!("ID not found")
                }
                i += 1;
            }
        }
        "3" => {
            println!("Enter ID of desired product of removal: ");
            let mut ID = String::new();
            io::stdin().read_line(&mut ID).expect("Failed to read input");
            let ID:u32 = ID.trim().parse().expect("Invalid ID");
            
            let mut i = 0;
            let mut id_exists = false;
            for (IDP, _, _) in &warehouse {
                if *IDP == ID {
                    id_exists = true;
                    warehouse.remove(i);
                    println!("Removed item {ID}");
                    break;
                } else {
                    println!("ID not found")
                }
                i += 1;
            }
        }
        "4" => {
            println!("{:?}", warehouse);
        }
        "5" => {
            println!("Exiting Program");
            break;
        }
        _ => {
            println!("Invalid choice");
        }

        }
    }

}
