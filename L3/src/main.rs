fn main() {
    let year = 2025; 
    let mut month = 7;

    //Reassigned month
    month = 12; 
    
    println!("year: {}, month:{}", year, month);

    let mut year = year;
    year = 2026;
    println!("New year: {}", year);

}
