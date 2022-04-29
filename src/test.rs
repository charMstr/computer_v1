// testing

use crate::equation_analyser;
use crate::equation_calculation;

#[test]
fn unit_init() {
    assert_eq!(2+2,4);
}

#[test]
fn unit_build_array_of_monome() {
    let side_equation_string = "3 * x^0 + 2 * x^1 - 4 * x^2";
    let test_vec = vec!("3*x^0","+2*x^1","-4*x^2");
    assert_eq!(equation_analyser::build_array_of_monome(side_equation_string),test_vec);

    let side_equation_string = "3  + 2 * x^1 - 4 * x^2 + 5*x^3  +8";
    let test_vec = vec!("3","+2*x^1","-4*x^2","+5*x^3","+8");
    assert_eq!(equation_analyser::build_array_of_monome(side_equation_string),test_vec);
}
/*
#[test]
fn unit_extract_signed_coefficient() {
    let monome = "-3*x^0".to_owned();
    let monome2 = "8".to_owned();
    let six:f32 = 6.0;
    let moins_trois:f32 = -3.0;
    assert_ne!(equation_analyser::extract_signed_coefficient(monome.clone()),six);
    assert_eq!(equation_analyser::extract_signed_coefficient(monome.clone()),moins_trois);
    assert_ne!(equation_analyser::extract_signed_coefficient(monome.clone()),-moins_trois);
    assert_eq!(equation_analyser::extract_signed_coefficient(monome2),8.00);
}*/

#[test]
fn unit_build_vector_of_signed_coefficients() {
    let input = vec!("3*x^0","+2*x^1","-4*x^2");
    let input: Vec<String> = input.into_iter().map(|s| s.to_string()).collect();
    let test_vec = vec!(3.0,2.0,-4.0);
    let test_vec2 = vec!(3.0,-2.0,-4.0);
    assert_eq!(equation_analyser::build_vector_of_signed_coefficients(input.clone()),test_vec);
    assert_ne!(equation_analyser::build_vector_of_signed_coefficients(input),test_vec2);
}

#[test]
fn unit_polynomial_degree() {

    //let test1 = "5* x^0 - 3*X^1-1*x^2   = -5*x^1-3*x^2";
    let test1 = "5 - 3*X^1 +2*x^2   = 3 -2*x^1";

    let split_equation1 : Vec<&str> = test1.split("=").collect();

    let mut equation = super::Equation {
        lhs: split_equation1[0].to_string(),
        rhs: split_equation1[1].to_string(),
        reduced_form : String::new(),
        delta:0.0
    };

    let array_of_monomes_left = equation_analyser::build_array_of_monome(&equation.lhs);
    let array_of_monomes_right = equation_analyser::build_array_of_monome(&equation.rhs);

    let mut array_of_signed_coeff_left = equation_analyser::build_vector_of_signed_coefficients(array_of_monomes_left);
    let array_of_signed_coeffs_right = equation_analyser::build_vector_of_signed_coefficients(array_of_monomes_right);

    equation.reduced_form = equation_analyser::create_reduced_form_of_equation(&mut array_of_signed_coeff_left,&array_of_signed_coeffs_right);

    let array_of_monomes_left = equation_analyser::build_array_of_monome(&equation.lhs);
    let array_of_monomes_right = equation_analyser::build_array_of_monome(&equation.rhs);

    let mut array_of_signed_coeff_left = equation_analyser::build_vector_of_signed_coefficients(array_of_monomes_left);
    let array_of_signed_coeffs_right = equation_analyser::build_vector_of_signed_coefficients(array_of_monomes_right);

    equation.reduced_form = equation_analyser::create_reduced_form_of_equation(&mut array_of_signed_coeff_left,&array_of_signed_coeffs_right);

    assert_eq!(equation_calculation::get_polynomial_degree(&equation.reduced_form),2);
}

#[test]
fn first_dataset() {

    
    let equation_string = "5 * x^0 + 4 * x^1 -9.3* x^2 = 1 * x^0";
    let split_equation : Vec<&str> = equation_string.split("=").collect();

    let mut equation = super::Equation {
        lhs: split_equation[0].to_string(),
        rhs: split_equation[1].to_string(),
        reduced_form : String::new(),
        delta:0.0
    };

    let array_of_monomes_left = equation_analyser::build_array_of_monome(&equation.lhs);
    let array_of_monomes_right = equation_analyser::build_array_of_monome(&equation.rhs);

    let mut array_of_signed_coeff_left = equation_analyser::build_vector_of_signed_coefficients(array_of_monomes_left);
    let array_of_signed_coeffs_right = equation_analyser::build_vector_of_signed_coefficients(array_of_monomes_right);

    equation.reduced_form = equation_analyser::create_reduced_form_of_equation(&mut array_of_signed_coeff_left,&array_of_signed_coeffs_right);

    assert_eq!(equation.reduced_form,"4.00 + 4.00 * X^1 - 9.30 * X^2 = 0");

    let polynomial_degree = equation_calculation::get_polynomial_degree(&mut equation.reduced_form);

    assert_eq!(polynomial_degree,2);

    assert_eq!(equation_calculation::get_nb_solution_second_degree(&mut equation,&array_of_signed_coeff_left),super::Solutions::TwoSolution);

    let response = vec!(0.90,-0.3651);
    let response:Vec<f32> = response.into_iter().map(|v:f32| v.round()).collect();
    let solving = equation_calculation::solve_second_degree_equation(&mut equation, &array_of_signed_coeff_left);
    let solving : Vec<f32> = solving.into_iter().map(|v:f32| v.round()).collect();

    assert_eq!(solving,response);
}