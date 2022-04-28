use std::env;

#[derive(Debug)]
struct Equation {
    lhs: String,
    rhs: String,
    reduced_form : String,
    delta : f32,
}

///this function will create a array of coefficients, 
fn build_vector_of_signed_coefficients(array_of_monome: Vec<String>) -> Vec<f32> //n°2
{
    let mut array_coefficients :Vec<f32> = Vec::new();
    for monome in array_of_monome {
        array_coefficients.push(extract_signed_coefficient(monome)); 
    }
    array_coefficients
}

/// This function will take as input a string representing a part of the polynome, as "a * X^y" and
/// return the coefficient with the correct sign.
///
/// example input1: "0*X^2"
/// example input2: "+1*X^3"
/// example input3: "-4*X^0"
fn extract_signed_coefficient(single_monome: String) -> f32 {
    //split until the '*' operator
    let coeff = single_monome.split("*").next().unwrap();
    let coeff = coeff.parse::<f32>().unwrap();
    //println!("signed_coeff: {}", coeff);
    coeff
}

/// this funciton takes as input a polynome
/// example: "a * X^0 - b * X^1"
/// and will return a vector of monomes, keeping the correct sign for the coefficient.
///
fn build_array_of_monome(polynome: &str) -> Vec<String> // n°1
{
    let mut array_of_monomes = Vec::new();

    let polynome = polynome.replace(" ", ""); 
    let polynome_with_extra_spaces = polynome.replace("+", " +");
    let polynome_with_extra_spaces = polynome_with_extra_spaces.replace("-", " -");
    let splited_in_monomes = polynome_with_extra_spaces.split(" ");
    for monome in splited_in_monomes {
        if monome.len() == 0 { //means there was an empty string because of a '-'
            continue
        }
        array_of_monomes.push(String::from(monome));
    }
    array_of_monomes
}

///this funciton will produce the reduced form of our equation for display on screen

fn create_reduced_form_of_equation(left: & mut Vec<f32>, right: &mut Vec<f32>) -> String { //n¨3

    let mut reduced = String::new();

    // /!\ modifies left in the function, &mut...
    coeff_reduced_form(left, right); 
    let left_reduced = left;
    println!("left_reduced : {:?}",left_reduced);

    for (index, coeff) in left_reduced.iter().enumerate()
    {  
        if coeff > &0.0 && index > 0 {
            let bit_of_equation = format!("+ {:.2} * X^{} ", coeff, index);
            reduced.push_str(&bit_of_equation); 
        }
        else {
            let bit_of_equation = format!("{:.2} * X^{} ", coeff, index);
            reduced.push_str(&bit_of_equation); 
        }
    }
    let mut reduced = reduced.replace('-', "- ");
    reduced.push_str("= 0");
    //println!("reduced equation: {}", reduced);
    reduced
}

///this funciton takes the coefficietns from the right hand side of the equation
/// and substracts them to the coefficients from the left hand side...
fn coeff_reduced_form(left: & mut Vec<f32>, right: &mut Vec<f32>) {
    for (l, r) in left.iter_mut().zip(right) {
        //println!("l = {}, and r = {}", l, r);
        *l = *l - *r;
    }
}

/// this function return the number of degree of a equation
fn get_polynomial_degree(reduced_form : &str) -> u32 {
    let polynomial_degree : usize = reduced_form.matches("X^").count();
    polynomial_degree as u32 -1
}

/// this function return the number of solution of a first degree equation
fn get_nb_solution_first_degree(array_coeff : &Vec<f32>) -> u16 {
    if array_coeff[0] != 0.0 {
        if array_coeff[1] != 0.0 {
            return 1;
        }
        else {
            return 50; // no solution
        }
    }
    else {
        if array_coeff[1] != 0.0 {
            return 0; // 0 is solution
        }
        else {
            return 99; // infinity of solution
        }
    }
}

/// this function return the number of solution of a second degree equation
fn get_nb_solution_second_degree(mut p_equation : &mut Equation, array_coeff : &Vec<f32>) -> u16 {
    p_equation.delta = (array_coeff[1]*array_coeff[1]) - (4.0*array_coeff[2] * array_coeff[0]);
    if p_equation.delta < 0.0 {
        0
    }
    else if p_equation.delta == 0.0 {
        1
    } else {
        2
    }
}
/* 
fn get_sqrt(number :f32) -> f32 {
    for i in 1..number/2 {
        if i*i == number {
            return i;
        }
    }
    panic!("no sqrt for this number");
}*/

/// this function take the array_coeff and the equation structure as an input and will return the solution of a equation of second degree
fn solve_second_degree_equation(p_equation : &mut Equation, array_coeff : &Vec<f32>) -> Vec<f32> {
    assert!(p_equation.delta >= 0.0);
    if get_nb_solution_second_degree(p_equation,array_coeff) == 2 {
        let mut res = Vec::new();
        res.push((-array_coeff[1]-f32::sqrt(p_equation.delta))/(2.0*array_coeff[2]));
        res.push((-array_coeff[1]+f32::sqrt(p_equation.delta))/(2.0*array_coeff[2]));
        res
    } else {
        let mut res = Vec::new();
        res.push((-array_coeff[1])/(2.0*array_coeff[2]));
        res
    }
}

/// this function take the array_coeff as an input and will return the solution of a equation of first degree
fn solve_first_degree_equation(array_coeff : &Vec<f32>) -> f32 {
    let res : f32 = -array_coeff[0] / array_coeff[1];
    res
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
    let split_around_equal : Vec<&str> = input.split("=").collect();

    let mut equation = Equation  
    {
        lhs: split_around_equal[0].to_string(),
        rhs: split_around_equal[1].to_string(),
        reduced_form: String::new(),
        delta:0.0,
    };

    let array_of_monomes_left = build_array_of_monome(&equation.lhs);
    let array_of_monomes_right = build_array_of_monome(&equation.rhs);

    let mut array_of_signed_coeff_left = build_vector_of_signed_coefficients(array_of_monomes_left);
    let mut array_of_signed_coeffs_right = build_vector_of_signed_coefficients(array_of_monomes_right);

    equation.reduced_form = create_reduced_form_of_equation(& mut array_of_signed_coeff_left, &mut array_of_signed_coeffs_right);

    println!("reduced form : {:?}",equation.reduced_form);
    //println!("mon delta = {} ",equation.delta);
    let polynomial_degree = match get_polynomial_degree(&mut equation.reduced_form) {
        0 => {println!("There is no unknown in your equation..");0},
        1 => {println!("polynomial degree : 1");1},
        2 => {println!("polynomial degree : 2");2},
        n @ 3.. => {println!("Sorry, we can't solve a polynomial equation of degree {} ",n); 0},
    };

    if polynomial_degree == 1 {
        match get_nb_solution_first_degree(&array_of_signed_coeff_left) {
            0 => println!("There is one solution which is 0"),
            1 => println!("The unique solution is : {} ",solve_first_degree_equation(&array_of_signed_coeff_left)),
            50 => println!("There is no solutions"),
            99 => println!("Infinity of solutions"),
            _ => panic!("bug in get_nb_solution_first_degree"),
        }
    }
    else if polynomial_degree == 2 {
        match get_nb_solution_second_degree(&mut equation, &array_of_signed_coeff_left) {
            0 => println!("There is no solution"),
            1 => println!("The unique solution is : {:?}",solve_second_degree_equation(&mut equation, &array_of_signed_coeff_left)),
            2 => println!("The two solutions are : {:?}",solve_second_degree_equation(&mut equation, &array_of_signed_coeff_left)),
            _ => panic!("bug in get_nb_solution_second_degree"),
        };
    }
}




// testing
#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        assert_eq!(2+2,4);
    }
}