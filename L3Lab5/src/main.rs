fn main() {
    let a = 10;
    {let b = a + 5; println!("{}", b); }
    let b = a + 5; //Since we declared b in the 
    println!("{}", b);
}
