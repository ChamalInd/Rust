// perform addition using 3 stacks
use std::io;

fn taking_inputs() -> (i32, i32) {
    loop {
        println!("Enter number 1:");
        let mut num1 = String::new();
        io::stdin()
            .read_line(&mut num1)
            .expect("Error while reading the number!");
        let num1: i32 = match num1.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter a correct number\n");
                continue;
            }
        };
        
       println!("Enter number 2:");
        let mut num2 = String::new();
        io::stdin()
            .read_line(&mut num2)
            .expect("Error while reading the number!");
        let num2: i32 = match num2.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter a correct number\n");
                continue;
            }
        };
        
        return (num1, num2);
    }
}

fn filling_stacks(arr: &mut [i32; 10], mut num: i32) {
    let mut index = 0;
    while num > 0 {
        arr[index] = num % 10;
        num /= 10;
        index += 1;
    }
}

fn perform_addition(ans: &mut [i32; 10], num1: &mut [i32; 10], num2: &mut [i32; 10]) {
    for index in 0..10 {
        let result = ans[index] + num1[index] + num2[index];

        ans[index] = result % 10;
        if index + 1 < 10 {
            ans[index + 1] = result / 10;
        }
    }
}

fn main() {
    loop {
        let (num1, num2) = taking_inputs();

        if num1 < 0 || num2 < 0 {
            println!("Exiting...");
            break;
        }

        let mut num1_arr: [i32; 10] = [0; 10];
        let mut num2_arr: [i32; 10] = [0; 10];
        let mut ans_arr: [i32; 10] = [0; 10];

        filling_stacks(&mut num1_arr, num1);
        filling_stacks(&mut num2_arr, num2);    
        perform_addition(&mut ans_arr, &mut num1_arr, &mut num2_arr);

        let mut num1_str = String::new();
        let mut num2_str = String::new();
        let mut ans_str = String::new();

        for i in (0..10).rev() {
            num1_str += &num1_arr[i].to_string();
            num2_str += &num2_arr[i].to_string();
            ans_str += &ans_arr[i].to_string();
        }

        println!("{num1_str} + {num2_str} = {ans_str}\n");
    }
}
