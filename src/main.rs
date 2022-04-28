use std::env;

mod equation_calculation;
mod equation_analyser;

#[derive(Debug)]
pub struct Equation {
    pub lhs: String,
    pub rhs: String,
    pub reduced_form : String,
    pub delta : f32,
}

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
       panic!("need one argument!\nusage: ./computer_v1 \"0 * X^0 + 1 * X^1 = 0\"");
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

    let array_of_monomes_left = equation_analyser::build_array_of_monome(&equation.lhs);
    let array_of_monomes_right = equation_analyser::build_array_of_monome(&equation.rhs);

    let mut array_of_signed_coeff_left = equation_analyser::build_vector_of_signed_coefficients(array_of_monomes_left);
    let array_of_signed_coeffs_right = equation_analyser::build_vector_of_signed_coefficients(array_of_monomes_right);

    equation.reduced_form = equation_analyser::create_reduced_form_of_equation(& mut array_of_signed_coeff_left, &array_of_signed_coeffs_right);

    println!("reduced form : {:?}",equation.reduced_form);
    //println!("mon delta = {} ",equation.delta);
    let polynomial_degree = equation_calculation::get_polynomial_degree(&mut equation.reduced_form);
    println!("polynomial degree : {}",polynomial_degree);

    if polynomial_degree == 1 {
        match equation_calculation::get_nb_solution_first_degree(&array_of_signed_coeff_left) {
            0 => println!("There is one solution which is 0"),
            1 => println!("The unique solution is : {} ",equation_calculation::solve_first_degree_equation(&array_of_signed_coeff_left)),
            50 => println!("There is no solutions"),
            99 => println!("Infinity of solutions"),
            _ => panic!("bug in get_nb_solution_first_degree"),
        }
    }
    else if polynomial_degree == 2 {
        match equation_calculation::get_nb_solution_second_degree(&mut equation, &array_of_signed_coeff_left) {
            0 => println!("There is no solution"),
            1 => println!("The unique solution is : {:?}",equation_calculation::solve_second_degree_equation(&mut equation, &array_of_signed_coeff_left)),
            2 => println!("The two solutions are : {:?}",equation_calculation::solve_second_degree_equation(&mut equation, &array_of_signed_coeff_left)),
            _ => panic!("bug in get_nb_solution_second_degree"),
        };
    } else if polynomial_degree > 2 {
        println!("Sorry, we can't solve polynomial degree more than degree 2");
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