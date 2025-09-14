use std::io::{self};

fn main() {
    let mut warehouse:Vec<(u32, String, u32)> = Vec::new(); //creating

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
            let mut new_id = String::new();
            io::stdin().read_line(&mut new_id).expect("Failed to read input");
            let new_id:u32 = new_id.trim().parse().expect("Invalid ID");

            let mut new_name = String::new();
            println!("Enter new product name: ");
            io::stdin().read_line(&mut new_name).expect("Failed to read input");
            let new_name = new_name.trim().to_string();
            
            let mut new_quantity = String::new();
            println!("Enter quantity: ");
            io::stdin().read_line(&mut new_quantity).expect("Failed to read input");
            let new_quantity:u32 = new_quantity.trim().parse().expect("Invalid quantity");

            warehouse.push((new_id, new_name, new_quantity));
            println!("Added new product");
        }

        "2" => {
            println!("Enter ID desired product to update quantity: ");
            let mut id = String::new();
            io::stdin().read_line(&mut id).expect("Failed to read input");
            let id:u32 = id.trim().parse().expect("Invalid ID");
            let mut i = 0;
            let mut id_exists = false;
            for (idp, _, _) in &warehouse {
                if *idp == id {
                    let mut new_quantity = String::new();
                    println!("Enter quantity: ");
                    io::stdin().read_line(&mut new_quantity).expect("Failed to read input");
                    let new_quantity:u32 = new_quantity.trim().parse().expect("Invalid quantity");
                    id_exists = true;
                    warehouse[i].2 = new_quantity; 
                    println!("Removed item {id}");
                    break;
                } else {
                    println!("ID not found")
                }
                i += 1;
            }
        }

        "3" => {
            println!("Enter ID of desired product of removal: ");
            let mut id = String::new();
            io::stdin().read_line(&mut id).expect("Failed to read input");
            let id:u32 = id.trim().parse().expect("Invalid ID");
            
            let mut i = 0;
            let mut id_exists = false;
            for (idp, _, _) in &warehouse {
                if *idp == id {
                    id_exists = true;
                    warehouse.remove(i);
                    println!("Removed item {id}");
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
