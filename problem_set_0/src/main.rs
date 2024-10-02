use std::io;

fn main () {
    indoor();
    // playback_speed();
}

// https://rusting.substack.com/p/handling-user-input
fn indoor () {
    let mut input = String::new();

    println!("Enter your name: ");
    
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    println!("{}", input.to_lowercase());
}

fn playback_speed () {
    let mut input = String::new();

    println!("Enter a sentencer with spaces: ");
    
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    println!("{}", input.replace(" ", "..."));
}