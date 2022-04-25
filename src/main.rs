use std::env;

#[derive(Debug)]
struct Equation {
    lhs: String,
    rhs: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
       panic!("usage: ./computer_v1 \"1 + X^2\"");
    }
    else if !args[1].contains("=") {
       panic!("equation must contains the \"=\" sign");
    }
    
    let input = args[1].clone();
    println!("{}", args[1]);
    let split_around_equal : Vec<&str> = input.split("=").collect();

    let equation = Equation  
    {
        lhs: split_around_equal[0].to_string(),
        rhs: split_around_equal[1].to_string(),
    };

    println!("{:?}", equation);
}
