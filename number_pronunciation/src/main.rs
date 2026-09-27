use std::io;

fn main() {
    loop {
        println!("Enter a number: ");
        let mut number = String::new();
        io::stdin()
            .read_line(&mut number)
            .expect("Error while reading the number!");
        let mut number: i32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Exiting...");
                break;
            }
        };
            
        if number > 999 {
            println!("Enter a number between -999 to 999\n");
            continue;
        }

        let mut number_name = String::new();
        let mut changed = false;
        let mut special = false;
    
        if number == 0 {
            number_name += "Zero";
        }

        if number < 0 {
            number_name += "Negative ";
            number = 0 - number;
        }

        if number >= 100 {
            let remainder = number / 100;
            number %= 100;
            changed = true;

            match remainder {
                1 => number_name += "One Hundred ",
                2 => number_name += "Two Hundred ",
                3 => number_name += "Three Hundred ",
                4 => number_name += "Four Hundred ",
                5 => number_name += "Five Hundred ",
                6 => number_name += "Six Hundred ",
                7 => number_name += "Seven Hundred ",
                8 => number_name += "Eight Hundred ",
                9 => number_name += "Nine Hundred ",
                _ => {
                    println!("Error occurred!");
                    break;
                }
            }
        }

        if number >= 10 {
            if changed {
                number_name += "and ";
            }

            let remainder = number / 10;
            number %= 10;
            changed = false;

            match remainder {
                2 => number_name += "Twenty ",
                3 => number_name += "Thirty ",
                4 => number_name += "Forty ",
                5 => number_name += "Fifty ",
                6 => number_name += "Sixty ",
                7 => number_name += "Seventy ",
                8 => number_name += "Eighty ",
                9 => number_name += "Ninety ",
                1 => {
                    special = true;
                    match number {
                        1 => number_name += "Eleven",
                        2 => number_name += "Twelve",
                        3 => number_name += "Thirteen",
                        4 => number_name += "Fourteen",
                        5 => number_name += "Fifteen",
                        6 => number_name += "Sixteen",
                        7 => number_name += "Seventeen",
                        8 => number_name += "Eighteen",
                        9 => number_name += "Nineteen",
                        _ => {
                            println!("Error occurred!");
                            break;
                        }
                    }
                },
                _ => {
                    println!("Error occurred!");
                    break;
                }
            }
        }
    
        if number > 0 && !special {
            if changed {
                number_name += "and ";
            }
        
            match number {
                1 => number_name += "One",
                2 => number_name += "Two",
                3 => number_name += "Three",
                4 => number_name += "Four",
                5 => number_name += "Five",
                6 => number_name += "Six",
                7 => number_name += "Seven",
                8 => number_name += "Eight",
                9 => number_name += "Nine",
                _ => {
                    println!("Error occurred!");
                    break;
                }
            }
        }

        println!("The number you entered is pronounced as {number_name}\n");
    }
}
