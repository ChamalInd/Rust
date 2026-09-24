use std::io;

fn main() {
    println!("Welcome to BMI Calculator!\n");

    println!("Enter weight: ");
    let mut weight = String::new();
    io::stdin()
        .read_line(&mut weight)
        .expect("Could not input weight.");
    let weight: f32 = weight.trim().parse().expect("Please enter a number.");

    println!("Enter height: ");
    let mut height = String::new();
    io::stdin()
        .read_line(&mut height)
        .expect("Could not input height.");
    let height: f32 = height.trim().parse().expect("Please enter a number.");

    let bmi = weight / (height * height);

    println!("\nWeight: {weight} Kg");
    println!("Height: {height} m");
    println!("BMI: {bmi}");

    match bmi {
        ..18.4 => println!("Bellow average BMI."),
        18.5..=24.9 => println!("Normal BMI."),
        _ => println!("Above average BMI."),
    };
}
