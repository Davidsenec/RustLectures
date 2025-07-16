fn main() {
    //PART 1
    let year = 2025; 
    let mut month = 7;

    //Reassigned month
    month = 12; 
    
    println!("Year: {}, Month:{}", year, month); 

    let mut year = year; //change mutability 
    year = 2026;
    println!("New year: {}", year);

    //PART 2
    let mut price = 99.99; 

    //assign 100 to price 
    price = 100.00; //we need to state .00 at the end as the price var's type is a floating point// price = 100; is wrong

    let discount_applied:bool = price < 100.00;

    println!("{discount_applied}");

    //PART 3
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

    //PART 4
    let mut score = 80;
    score += 10; //adds 10 to le score
    let score = score > 85; //shadows score and changes its type and mutability
    let score = if score {"Passed"} else {"Failed"}; 
    println!("score: {}", score); 

    //PART 5
    let a = 10;
    let mut b = 5;
    {
        b = a + 5; println!("{}", b);
        let mut a = 10.0; //changes 'a' into float in the inner scope
        a += 0.5;
        println!("{}", a);
     }

    //let b = a + 5; //Since we declared b in the scope, we have to shadow the value to print; here a is back to its original value
    println!("b is now: {}", b);
    println!("a is now: {}", a);
}
