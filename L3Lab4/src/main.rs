fn main() {
    let mut score = 80;
    score += 10;
    let score = score > 85;
    let score = if score {"Passed"} else {"Failed"};
    println!("score: {}", score); 

}
