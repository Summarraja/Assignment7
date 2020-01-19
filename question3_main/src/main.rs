use question3_library::fun;
use std::io;
fn main() {
    
    let mut input = String::new();
    let name: String;
    
    println!("Enter your name");
    
    io::stdin().read_line(&mut input).expect("Failed to get input");
    name = input.trim().parse().expect("Invalid name entered");
    
    let num = fun::tool::lucky_number(name.clone());
    println!("{} your lucky number is {}",name,num);
}
