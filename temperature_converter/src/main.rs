use std::io;

fn main() {
    println!("Temperature Converter\n1 - Celsius to Fahrenheit\n2 - Fahrenheit to Celsius\n3 - Exit\n");

    loop {
        println!("Enter a operation: ");
        
        let mut operation = String::new();
        io::stdin()
            .read_line(&mut operation)
            .expect("Error while reading the operation!");
        let operation: i32 = match operation.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Incorrect operation.\n");
                continue;
            }
        };

        if operation < 1 || operation > 3 {
            println!("Incorrect operation.\n");
            continue;

        } else if operation == 3 {
            println!("Exiting...");
            break;

        }
        
        println!("Enter the value: ");
        
        let mut value = String::new();
        io::stdin()
            .read_line(&mut value)
            .expect("Error while reading the value!");
        let value: f32 = match value.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Incorrect number\n");
                continue;
            }
        };

        if operation == 1 {
            let result = celsius_to_fahrenheit(value);
            println!("Celsius: {value} C\nFahrenheit: {result} F\n");
        
        } else {
            let result = fahrenheit_to_celsius(value);
            println!("Fahrenheit: {value} F\nCelsius: {result} C\n");
        }
    }
}

fn celsius_to_fahrenheit(value: f32) -> f32 {
    (9.0 / 5.0 * value) + 32.0 
}

fn fahrenheit_to_celsius(value: f32) -> f32 {
    (value - 32.0) * (5.0 / 9.0)
}
