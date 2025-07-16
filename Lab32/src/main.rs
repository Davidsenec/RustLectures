fn main() {
    let base_price = 150.0;
    let mut discount = 0.0;

    let is_student = true; //applies student discount
    let is_early_bird = false; //applies early bird discount
    let has_coupon = true; // applies coupon
    let free_entry = false; //allows free entry

    if is_student {discount += 0.10}; 
    if is_early_bird {discount += 0.20};
    if has_coupon {discount += 0.05};

    let final_price = base_price * (1.00 - discount);
    let free_entry = {
        let free_entry = final_price < 50.0;
        free_entry
        }; 

    println!("Base ticket price: {}", base_price);
    println!("Student discount applied: {}", is_student);
    println!("Early bird discount applied: {}", is_early_bird);
    println!("Coupon used: {}", has_coupon);
    println!("Final ticket price {}", final_price);
    println!("Free entry: {}", free_entry); 
}
