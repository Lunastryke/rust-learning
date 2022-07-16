use std::io;
fn main() {
    // When main() function goes out of scope, drop() function will be called on user_input object to deallocate it
    let mut user_input = String::new();
    // let s1 = &user_input;
	// let s2 = &user_input;
	// This mixing of immutable and mutable references is allowed as the compiler knows
	// we do not use s1 and s2 after mutating it. 
	// println!("{} , {}", s1, s2);
    println!("Enter your weight (kg): ");
    // unwrap() will terminate the program if the function call fails
    io::stdin().read_line(&mut user_input).unwrap();
    let weight: f32 = user_input.trim().parse().unwrap();
    dbg!(weight);
    println!("user input: {}", user_input);
    // However if we put the println!() here an error will be thrown
    // println!("{} , {}", s1, s2);
    // Type of variable does not need to be specified, compiler will figure it out
    let mut mars_weight = calculate_weight_on_mars(weight);
    mars_weight = mars_weight * 1000.0;
    // ! indicates macro definitions, can receive variable number of variables and variable types
    println!("Weight on Mars: {}kg", mars_weight);
}

// Semicolon omitted -> Statement will be returned
fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}
