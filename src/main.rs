use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
       panic!("usage: ./computer_v1 \"1 + X^2\"");
    }
    
    let input = args[1].clone();
    println!("{}, {}", input, args[1]);
}
