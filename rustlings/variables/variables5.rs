// variables5.rs
// Make me compile! Execute the command `rustlings hint variables5` if you want a hint :)

fn main() {
    let number = "T-H-R-E-E"; // &str
    println!("Spell a Number : {}", number);
    // use variable shadow
    let number = 3; // mismatched types; Found i32 Expected &str
    println!("Number plus two is : {}", number + 2);
}
