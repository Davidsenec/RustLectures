fn main() {
    let mut price = 99.99;

    //assign 100 to price 
    price = 100.00; //we need to state .00 at the end as the price var's type is a floating point; price = 100; is wrong

    let discount_applied:bool = price < 100.00;

    println!("{discount_applied}");
}
