use std::io;

fn main() {
    let mut binary: [u32; 8] = [0; 8];
    
    for i in 0..8 {
        binary[i] = take_input(i as u8);
    }
    
    let decimal: u32 = binary_to_decimal(binary);

    println!("{binary:?} = {decimal}");
}

fn binary_to_decimal(binary: [u32; 8]) -> u32 {
    let mut decimal: u32 = 0;

    for i in 0..8 {
        decimal = decimal * 2 + binary[i];
    }

    return decimal;
}

fn take_input(index: u8) -> u32 {
    loop {
        println!("Enter a number for element {}: (0/1)", index + 1);
        let mut num = String::new();
        io::stdin()
            .read_line(&mut num)
            .expect("Error occurred while reading the number\n");
        let num: u32 = match num.trim().parse() {
            Ok(num) => {
                match num {
                    0 => 0,
                    1 => 1,
                    _ => {
                        println!("Enter a correct number\n");
                        continue;
                    }
                }
            },
            Err(_) => {
                println!("Enter a correct number\n");
                continue;
            }
        };

        return num;
    }
}
