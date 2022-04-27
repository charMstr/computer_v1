use std::env;

#[derive(Debug)]
struct Equation {
    lhs: String,
    rhs: String,
    reduce_form : String,
    delta : f32,
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

    let mut equation = Equation  
    {
        lhs: split_around_equal[0].to_string(),
        rhs: split_around_equal[1].to_string(),
        reduce_form: String::new(),
        delta:0.0,
    };

    let mut right_coeff : [f32;3] = [0.0,0.0,0.0]; // mind equation > 2 degree
    let mut left_coeff : [f32;3] = [0.0,0.0,0.0];


    let split_right = equation.rhs.replace(" ","");
    let split_right : Vec<&str> = split_right.split(&['+', '-']).collect();
    // for every pieces, we're going to test the presence of the right format (mind uppercase)
    for (i,elem) in split_right.iter().enumerate() {
        if elem.contains(format!("x^{}",i).as_str()) {
            let temp : Vec<&str> = elem.split('*').collect();
            right_coeff[i] = temp[0].parse().unwrap();
        }
        else {
            panic!("mauvais formatage, vous devez écrire : a * x^0 + b * x^1 .... même si le coeff est nul");
        }
        
    }

    let split_left = equation.lhs.replace(" ", "");
    let split_left : Vec<&str> = split_left.split(&['+', '-']).collect();

    for (i,elem) in split_left.iter().enumerate() {
        if elem.contains(format!("x^{}",i).as_str()) { // mind uppercase of x !
            let temp : Vec<&str> = elem.split('*').collect();
            left_coeff[i] = temp[0].parse().unwrap();

        }
        else {
            panic!("mauvais formatage, vous devez écrire : a * x^0 + b * x^1 .... même si le coeff est nul");
        }
    }

    // found the negative sign of left part
    for i in 0..split_left.len() {
        let equation_string = equation.lhs.replace(" ",""); // i have to assign a varialbe here, otherwise it will be drop at the end of the statement
        //let split_left_byte = equation.lhs.replace(" ","").as_bytes();
        let split_left_byte = equation_string.as_bytes();
        let index_number = match equation.lhs.replace(" ", "").find(split_left[i]) {
            Some(res) => res,
            None => 0,
        };
        if index_number > 0 {
            if split_left_byte[index_number-1] == 45 {
                //println!("here there is a negativ sign : {}",index_number-1);
                left_coeff[i] = -left_coeff[i];
            }
        }
    }

    // found the negative sign of right part
    for i in 0..split_right.len() {
        let equation_string = equation.rhs.replace(" ","");
        let split_right_byte = equation_string.as_bytes();
        let index_number = match equation.rhs.replace(" ", "").find(split_right[i]) {
            Some(res) => res,
            None => 0,
        };
        if index_number > 0 {
            if split_right_byte[index_number-1] == 45 {
                right_coeff[i] = -right_coeff[i];
            }
        }
    }

    println!("at the end : left coeff : {:?}  = right coeff : {:?}",left_coeff, right_coeff);
    let mut reduce_coeff :[f32;3] = [0.0,0.0,0.0];

    for i in 0..reduce_coeff.len() {
        reduce_coeff[i] = left_coeff[i] - right_coeff[i];
    }
    // working solution as well but better use the simple one :)
    /*for ((x,left),right) in reduce_coeff.iter_mut().zip(left_coeff).zip(right_coeff) {
        *x = left- right;
    }*/
    println!("reduce coeff : {:?}",reduce_coeff);
    equation.reduce_form = format!("{} * X^0 + {} * X^1 + {} * X^2 = 0",reduce_coeff[0],reduce_coeff[1],reduce_coeff[2]);
    println!("reduce form  : {:?}",equation.reduce_form);

    // mind to use the delta calculation only for 2nd polynomial degree 
    
    equation.delta = (reduce_coeff[1]*reduce_coeff[1]) - (4.*reduce_coeff[2]*reduce_coeff[0]);
    println!("delta =  {:?}",equation.delta);
    // Display of the number/solutions 
    if equation.delta < 0.0 {
        println!("Il n'y a aucune solution pour résoudre cette Equation dans R");
        println!("pas de solutions");
    }
    else if equation.delta == 0.0 {
        println!("Il y a une solution à cette équation");
        println!("la solution est : {}",-reduce_coeff[1]/(2.0*reduce_coeff[2]));
    }
    else {
        println!("Il y a 2 solutions distinctes à cette équation");
        println!("les solutions sont : {} et {}",(-reduce_coeff[1]+f32::sqrt(equation.delta))/(2.0*reduce_coeff[2]),(-reduce_coeff[1]-f32::sqrt(equation.delta))/(2.0*reduce_coeff[2]));
    }

    // display of the polynomial degree
    if reduce_coeff[2] != 0.0 {
        println!("Polynomia degree : 2");
    }
    else if reduce_coeff[1] != 0.0 {
        println!("Polynomia degree : 1");
    }
    else if reduce_coeff[0] != 0.0 {
        println!("Polynomia degree : 0");
    }
    else {
        println!("pas d'inconnu dans cette équation...");
    }

}
