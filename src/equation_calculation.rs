use crate::Equation;
/// this function return the number of degree of a equation
pub fn get_polynomial_degree(reduced_form : &str) -> u32 {
    let polynomial_degree : usize = reduced_form.matches("X^").count();
    polynomial_degree as u32 -1
}

/// this function return the number of solution of a first degree equation
pub fn get_nb_solution_first_degree(array_coeff : &Vec<f32>) -> u16 {
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
pub fn get_nb_solution_second_degree(mut p_equation : &mut Equation, array_coeff : &Vec<f32>) -> u16 {
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
/// this function take the array_coeff and the equation structure as an input and will return the solution of a equation of second degree
pub fn solve_second_degree_equation(p_equation : &mut Equation, array_coeff : &Vec<f32>) -> Vec<f32> {
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
pub fn solve_first_degree_equation(array_coeff : &Vec<f32>) -> f32 {
    let res : f32 = -array_coeff[0] / array_coeff[1];
    res
}