fn main() {
    let available = true;
    let mut in_stock = false;
    let rating = 4.5;

    let mut is_good = available && rating > 4.0 && in_stock;
    println!("{}", is_good);
    
    in_stock = true;
    is_good = available && rating > 4.0 && in_stock;   
    println!("{}", is_good);

    println!("Opposite of available: {}", !available);
    println!("Available or in stock: {}", available || in_stock);
    println!("Rating is less than 3 or more than 4: {}", rating < 3.0 || rating > 4.0);

}
