/// this function will create a vec of float coefficients build by a vec of str coefficients
/// input --> vec!["3";"+2*x";"-4*x^3"]   output --> vec![3;2;0;-4] 
pub fn build_vector_of_signed_coefficients(array_of_monome: Vec<String>) -> Vec<f32> //n°2
{
    let mut array_coefficients :Vec<f32> = Vec::new();
    /* 
    for monome in array_of_monome {
        array_coefficients.push(extract_signed_coefficient(monome)); 
    }*/
    for monome in array_of_monome {
        let pieces: Vec<&str> = monome.split("*").collect();
        //println!("mes pieces : {:?}",pieces);
        if pieces.len() > 1 { // if we have a time operator
            if pieces[1].contains("x^") || pieces[1].contains("X^") { // let's check for an x or X (indeed it could be 3*5)
                let byte_string = pieces[1].as_bytes();    
                //println!("mon tableau de byte : {:?}",byte_string);
                let index_puissance = pieces[1].find('^').unwrap()+1;
                let puissance = byte_string[index_puissance] as char;
                //println!("ma puissance  as char !!:  : {:?}",puissance);
                let puissance = puissance.to_digit(35).unwrap();
                let puissance = puissance as usize;
                //println!("ma puissance as f32 :  : {:?}",puissance as f32);
                if array_coefficients.len() < puissance+1 {
                    array_coefficients.resize(puissance+1, 0.0);
                }
                array_coefficients[puissance] += pieces[0].parse::<f32>().unwrap();
            } else {
                // we have to make the operation beetween the 2 number and push it
                let product = pieces[0].parse::<f32>().unwrap() * pieces[1].parse::<f32>().unwrap();
                array_coefficients[0] = product
            }
            
        } else {
            if array_coefficients.len() == 0 {
                array_coefficients.push(pieces[0].parse::<f32>().unwrap());
            }else {
                array_coefficients[0] = pieces[0].parse::<f32>().unwrap();
            }
        }
    }
    //println!("mon vec signed : {:?}",array_coefficients);
    array_coefficients
}
/* 
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
}*/

/// this function will create an array of coefficients build by the equation string input
/// input --> "3 + 2*x - 4x^3 = 0"  output --> vec!["3";"+2*x";"-4*x^3"]
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
        else if coeff < 0.0 && index > 0 {
            let bit_of_equation = format!("{:.2} * X^{} ", coeff, index);
            reduced.push_str(&bit_of_equation); 
        }
        else if coeff == 0.0 {

        }
        else if index == 0 {
            let bit_of_equation = format!("{:.2} ", coeff);
            reduced.push_str(&bit_of_equation); 
        }
        else {
            let bit_of_equation = format!("{:.2} * X^{} ", coeff, index);
            reduced.push_str(&bit_of_equation); 
        }        
    }
    reduced = reduced.replace('-', "- ");
    //let mut reduced = reduced.replace("^1",""); // can't do this operation now or it will bug further
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

/// this function will return true if the equation has a correct format, and false if it's not the case
/// only allowing digit or 'x' and 'X' char in the input and the symbol '+' '-' '*' '^' '='
pub fn is_input_correct(equation: &String) -> bool {

    //println!("input : {}",equation);
    // syntaxe checking
    for char in equation.chars() {
        // only number or '+' '-' '*'and '=' or 'x' symbol allowed 
        if !char.is_ascii_digit() && char != '+' && char != '-' && char != '.' && char != '^' && char != '*' && char != '=' && char != 'x' && char != 'X' {
            return false;
        }
        //println!("mon char ! {}",char);
    }

    let equation_bytes = equation.as_bytes();
    //println!("bytes : {:?}",equation_bytes);
    // checking for a digit or a 'x' after '+' '-' or '*'  symbol
    for (i,byte) in equation_bytes.iter().enumerate() {
        if i == equation_bytes.len()-1 { // checking for something correct at the last index
            if !equation_bytes[i].is_ascii_digit() && equation_bytes[i] != 120 && equation_bytes[i] != 88 {
                panic!("erreur, you have to put something after the \"=\" sign !");
            }
            return true;
        }
        if (byte == &43 || byte == &45 || byte == &42) && (!equation_bytes[i+1].is_ascii_digit() && equation_bytes[i+1] != 120 && equation_bytes[i+1] != 88)  {
            return false;
        }
        else if byte == &94 && !equation_bytes[i+1].is_ascii_digit() { // checking for digit after '^' symbol
            return false;
        }
        else if byte == &61 && !equation_bytes[i+1].is_ascii() { // checking that not empty after = symbol
            return false;
        }
        else if (byte == &120 || byte == &88) && equation_bytes[i+1] != 94 { // checking for '^' after X or x 
            return false;
        }
    }
    true
}


/// this function will transform the string intput by adding the * between the digit and x symbol
/// it will also transform your 'x' into "x^1", and you're x into 1*x so you can write in normal form your equaiton
/// example : input -> "3 +2*x -2x^2= 81" output -> "3+2*x^1-2*x^2=81"
pub fn correct_input(input :&mut String) {

    *input = input.replace(" ","");

    let mut indexes_x: Vec<usize> = Vec::new();
    let mut indexes_star: Vec<usize> = Vec::new();

    let mut indexes_pow: Vec<usize> = Vec::new();

    // managing the '*' add
    for (i,char) in input.chars().enumerate() {
        if (char == 'x' || char == 'X') && i > 0 {
            if input.chars().nth(i-1).unwrap().is_digit(35) {
                indexes_x.push(i);
            }
            else if input.chars().nth(i-1).unwrap() != '*' {
                indexes_star.push(i);
            }
        }
    }
    // inserting * if needed
    for i in 0..indexes_x.len() {
        let index = indexes_x[i];
        input.insert(index,'*');
        for x in &mut indexes_x {
            *x += 1;
        }
        for x in &mut indexes_star {
            *x += 1;
        }
    }
    // inserting 1* if needed
    for i in 0..indexes_star.len() {
        let index = indexes_star[i];
        input.insert(index,'*');
        input.insert(index,'1');
        for x in &mut indexes_star {
            *x += 2;
        }
        for x in &mut indexes_x {
            *x += 2;
        }
    }


    // managing the '^1' add
    for (i,char) in input.chars().enumerate() {
        if char == 'x' || char == 'X' {
            if input.chars().nth(i+1).is_none() { // in case x is our last char
                indexes_pow.push(i+1);
                break;
            }
            if input.chars().nth(i+1).unwrap() != '^' {
                indexes_pow.push(i+1);
            }
            
        }
    }
    //inserting the ^1
    for i in 0..indexes_pow.len() {
        let index = indexes_pow[i];
        input.insert(index, '^');
        input.insert(index+1,'1');
        for x in &mut indexes_pow {
            *x += 2;
        }
    }
}