use std::io;
mod echo;
use echo::echo;

fn main() {
    println!("Hello, world!");


    let mut to_echo = String::new();

    io::stdin()
      .read_line(&mut to_echo)
      .expect("Failed to read line");
    
    echo(to_echo);
}
