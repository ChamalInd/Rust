use std::io;
use std::f32::consts::PI; 

fn main() {
    print_menu();
    loop {
        println!("Enter a menu item: ");
        match taking_inputs() {
            1.0 => simple_arithmatics('+'),
            2.0 => simple_arithmatics('-'),
            3.0 => simple_arithmatics('*'),
            4.0 => simple_arithmatics('/'),
            5.0 => power_of(),
            6.0 => lcm_gcd_calculator("lcm".to_string()),
            7.0 => lcm_gcd_calculator("gcd".to_string()),
            8.0 => trig_calculations("sin".to_string()),
            9.0 => trig_calculations("cos".to_string()),
            10.0 => trig_calculations("tan".to_string()),
            11.0 => inverse(),
            12.0 => simple_arithmatics('%'),
            13.0 => odd_even(),
            14.0 => percentage(),
            15.0 => {
                println!("Exiting...");
                break;
            },
            _ => {
                println!("Enter a valid input.\n");
                continue;
            }
        }
    }
}

fn percentage() {
    println!("Enter a number:");
    let num: f32 = taking_inputs();

    println!("{num}% = {}\n", num / 100.0);
}

fn odd_even() {
    println!("Enter a number:");
    let num: f32 = taking_inputs();
    
    if num % 2.0 == 0.0 {
        println!("{num} is an even number.\n");
    } else {
        println!("{num} is an odd number.\n");
    }
}

fn inverse() {
    println!("Enter a number:");
    let num: f32 = taking_inputs();

    println!("1/{num} = {}\n", 1.0 / num);
}

fn trig_calculations(oper: String) {
    println!("Enter an angle: ");
    let ang: f32 = taking_inputs();
    let rad: f32 = ang * (PI / 180.0);

    match oper.as_str() {
        "sin" => println!("sin({ang}) = {}\n", rad.sin()),
        "cos" => println!("cos({ang}) = {}\n", rad.cos()),
        "tan" => println!("tan({ang}) = {}\n", rad.tan()),
        _ => println!("Error occurred while calculating.\n")
    }
}

fn euclidean_algorithm_for_gcd(num1: i32, num2: i32) -> i32 {
    let rem: i32 = num1 % num2;
    
    if rem == 0 {
        return num2;
    } else {
        return euclidean_algorithm_for_gcd(num2, rem);
    }
}

fn lcm_gcd_calculator(oper: String) {
    println!("Enter the number 1:");
    let num1: i32 = taking_inputs() as i32;
    println!("Enter the number 2:");
    let num2: i32 = taking_inputs() as i32;
    
    let gcd: i32;

    if num1 > num2 {
        gcd = euclidean_algorithm_for_gcd(num1, num2);
    } else {
        gcd = euclidean_algorithm_for_gcd(num2, num1);
    }

    match oper.as_str() {
        "lcm" => {
            println!("LCM of {num1} and {num2} is {}\n", (num1 * num2) / gcd);
        },
        "gcd" => {
            println!("GCD of {num1} and {num2} is {gcd}\n");
        },
        _ => println!("Error occurred while calculating.\n")
    }
}

fn power_of() {
    println!("Enter the base:");
    let base: f32 = taking_inputs();
    println!("Enter the power:");
    let power: i32 = taking_inputs() as i32;

    println!("{base} ^ {power} = {}\n", base.powi(power));
}

fn simple_arithmatics(oper: char) {
    println!("Enter the number 1:");
    let num1: f32 = taking_inputs();
    println!("Enter the number 2:");
    let num2: f32 = taking_inputs();

    match oper {
        '+' => println!("{num1} + {num2} = {}\n", num1 + num2),
        '-' => println!("{num1} - {num2} = {}\n", num1 - num2),
        '/' => println!("{num1} / {num2} = {}\n", num1 / num2),
        '*' => println!("{num1} * {num2} = {}\n", num1 * num2),
        '%' => println!("{num1} % {num2} = {}\n", num1 % num2),
        _ => println!("Error occurred while calculating.\n")
    }
}

fn taking_inputs() -> f32 {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error occurred while reading the input.\n");
        let input: f32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter a valid input.\n");
                continue;
            }
        };

        return input;
    }
}

fn print_menu() {
    println!("{}\n##{:^38}##\n##{:^38}##\n##{:^38}##\n{}\n{}", "#".repeat(42), "Scientific Calculator", "by", "Chamal Induwara", "#".repeat(42), "#".repeat(42));
    println!("#  {:<18} {:<18} #", " 1. Addition", " 8. Sine");
    println!("#  {:<18} {:<18} #", " 2. Subtraction", " 9. Cosine");
    println!("#  {:<18} {:<18} #", " 3. Multiplication", "10. Tangent");
    println!("#  {:<18} {:<18} #", " 4. Division", "11. Inverse");
    println!("#  {:<18} {:<18} #", " 5. Power", "12. Remainder");
    println!("#  {:<18} {:<18} #", " 6. LCM", "13. Odd or Even");
    println!("#  {:<18} {:<18} #", " 7. GCD", "14. Percentage");
    println!("# {:^38} #", " ");
    println!("#  {:<38}#", "15. Exit");
    println!("{}\n{}\n", "#".repeat(42), "#".repeat(42));
}
