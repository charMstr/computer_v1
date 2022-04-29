///this function will create a array of coefficients, 
pub fn build_vector_of_signed_coefficients(array_of_monome: Vec<String>) -> Vec<f32> //n°2
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
pub fn extract_signed_coefficient(single_monome: String) -> f32 {
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
pub fn build_array_of_monome(polynome: &str) -> Vec<String> // n°1
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
pub fn create_reduced_form_of_equation(left: & mut Vec<f32>, right: &Vec<f32>) -> String { //n¨3

    let mut reduced = String::new();

    coeff_reduced_form(left, right); // /!\ modifies left in the function
    let left_reduced = left;
    //println!("left_reduced : {:?}",left_reduced);

    for (index, &coeff) in left_reduced.iter().enumerate()
    {  
        if coeff > 0.0 && index > 0 {
            let bit_of_equation = format!("+ {:.2} * X^{} ", coeff, index);
            reduced.push_str(&bit_of_equation); 
        }
        else if coeff == 0.0 {
            
        }
        else {
            let bit_of_equation = format!("{:.2} ", coeff);
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
pub fn coeff_reduced_form(left: & mut Vec<f32>, right: &Vec<f32>) {
    for (l, r) in left.iter_mut().zip(right) {
        //println!("l = {}, and r = {}", l, r);
        *l = *l - *r;
    }
}