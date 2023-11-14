// Program that calculates the weight of a person in different planets.
// Author: Diogo Esteves
//
// Usage: cargo run
//

use std::io;

fn main (){
    println!("Enter the planet you want to know your weight:");
    let mut p_input = String::new();
    io::stdin().read_line(&mut p_input).unwrap();

    println!("Enter your weight:");
    let mut w_input = String::new();
    io::stdin().read_line(&mut w_input).unwrap();

    let weight: f32 = w_input.trim().parse().unwrap();
    let planet: &str = p_input.trim();

    let planet_cap = uppercase_words(planet).to_string();
    let weight_on_planet = calculate_weight_on_planet(weight, &planet_cap);

    println!("Weight on planet {} is {}kg.", planet_cap, weight_on_planet);
}

fn uppercase_words(data: &str) -> String {
    // Uppercase words
    let mut result = String::new();
    for value in data.chars() {
        result.push(value.to_ascii_uppercase());
    }
    result
}

fn calculate_weight_on_planet(weight: f32, planet: &str) -> f32 {
    // Calculate weight on planet
    match planet {
        "MERCURY" => {
            weight * 0.38
        }
        "VENUS" => {
            weight * 0.91
        }
        "EARTH" => {
            weight
        } 
        "MARS" => {
            weight * 0.38
        } 
        "JUPITER" => {
            weight * 2.34
        } 
        "SATURN" => {
            weight * 1.06
        } 
        "URANUS" => {
            weight * 0.92
        } 
        "NEPTUNE" => {
            weight * 1.19 
        }
        _ => {
            0.0
        }
    }
}
