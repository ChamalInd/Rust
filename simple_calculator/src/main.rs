use std::io;

fn main() {
    println!("Simple calculator\n");

    loop {
        println!("Enter the operator:");
        let mut operator = String::new();
        io::stdin()
            .read_line(&mut operator)
            .expect("Error while trying to input the operator!");
        let operator: char = match operator.trim().parse() {
            Ok(oper) => oper,
            Err(_) => {
                println!("Exiting...");
                break;
            }
        };
        
        match operator {
            '+' => println!("Performing addition..."),
            '-' => println!("Performing subtraction..."),
            '*' => println!("Performing multiplication..."),
            '/' => println!("Performing division..."),
            _ => { 
                println!("Exiting...");
                break;
            }
        }

        println!("Enter the number 1: ");
        let mut num1 = String::new();
        io::stdin()
            .read_line(&mut num1)
            .expect("Error while trying to input the number!");
        let num1: f32 = match num1.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter a correct number");
                continue;
            }
        };

        println!("Enter the number 2: ");
        let mut num2 = String::new();
        io::stdin()
            .read_line(&mut num2)
            .expect("Error while trying to input the number!");
        let num2: f32 = match num2.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter a correct number");
                continue;
            }
        };

        match operator {
            '+' => println!("{num1} + {num2} = {}\n", num1 + num2),
            '-' => println!("{num1} - {num2} = {}\n", num1 - num2),
            '*' => println!("{num1} * {num2} = {}\n", num1 * num2),
            '/' => println!("{num1} / {num2} = {}\n", num1 / num2),
            _ => println!("Error occurred while calculating")
        }
    }
}
