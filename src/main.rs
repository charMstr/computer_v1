use std::env;

mod equation_calculation;
mod equation_analyser;

#[cfg(test)]
mod test;

#[derive(Debug)]
pub struct Equation {
    pub lhs: String,
    pub rhs: String,
    pub reduced_form : String,
    pub delta : f32,
}

#[derive(PartialEq)]
#[derive(Debug)]
 pub enum Solutions {
    NoSolution,
    ZeroOnlySolution,
    UniqueSolution,
    TwoSolution,
    InfinityOfSolution,
}

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
       panic!("need one argument!\nusage: ./computer_v1 \"0 * X^0 + 1 * X^1 = 0\"");
    }
    else if !args[1].contains("=") {
       panic!("equation must contains the \"=\" sign");
    }
    
    let mut input = args[1].clone();
    //println!("avant correction : {:?}",input);
    equation_analyser::correct_input(&mut input);
    //println!("apres correction : {:?}",input);
    let test = equation_analyser::is_input_correct(&mut input);
    if !test {
        panic!("input not OK");
    }

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

    let reduced_form = equation.reduced_form.replace("^1", "");
    println!("reduced form : {:?}",reduced_form);

    //println!("mon delta = {} ",equation.delta);
    let polynomial_degree = equation_calculation::get_polynomial_degree(&mut equation.reduced_form);
    println!("polynomial degree : {}",polynomial_degree);

    if polynomial_degree == 1 {
        match equation_calculation::get_nb_solution_first_degree(&array_of_signed_coeff_left) {
            Solutions::ZeroOnlySolution => println!("There is one solution which is 0"),
            Solutions::UniqueSolution => println!("The unique solution is : {} ",equation_calculation::solve_first_degree_equation(&array_of_signed_coeff_left)),
            Solutions::NoSolution => println!("There is no solutions"),
            Solutions::InfinityOfSolution => println!("Infinity of solutions"),
            _ => panic!("bug in get_nb_solution_first_degree"),
        }
    }
    else if polynomial_degree == 2 {
        match equation_calculation::get_nb_solution_second_degree(&mut equation, &array_of_signed_coeff_left) {
            Solutions::NoSolution => println!("There is no solution in R"),
            Solutions::UniqueSolution => println!("The unique solution is : {:?}",equation_calculation::solve_second_degree_equation(&mut equation, &array_of_signed_coeff_left)),
            Solutions::TwoSolution => println!("The two solutions are : {:?}",equation_calculation::solve_second_degree_equation(&mut equation, &array_of_signed_coeff_left)),
            _ => panic!("bug in get_nb_solution_second_degree"),
        };
    } else if polynomial_degree > 2 {
        println!("Sorry, we can't solve polynomial degree more than degree 2");
    } else {
        println!("Votre equation ne contient pas d'inconnu...");
    }
}