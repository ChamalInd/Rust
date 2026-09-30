use std::io;

fn operation_inputs() -> i32 {
    println!("1: push\n2: pop\n3: peek\n4: print\n");
    println!("Enter an operation: ");
    let mut operation = String::new();
    io::stdin()
        .read_line(&mut operation)
        .expect("Unable to read the operation");
    let operation: i32 = match operation.trim().parse() {
        Ok(num) => num,
        Err(_) => -1
    };

    return operation;
}

fn taking_inputs() -> i32 {
    loop {
        println!("Enter a number: ");
        let mut number = String::new();
        io::stdin()
            .read_line(&mut number)
            .expect("Unable to read the number");
        let number: i32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter a valid number");
                continue;
            }
        };

        return number;
    }
}

fn push(arr: &mut [i32; 10], top: &mut usize, num: i32) {
    if *top + 1 >= 10 {
        println!("Stack is full...\n");
        return;
    }
    
    arr[*top] = num;
    *top += 1;
    println!("Pushing {num} to {}\n", *top);
}

fn pop(arr: &mut [i32; 10], top: &mut usize) {
    if *top <= 0 {
        println!("Stack is empty...\n");
        return;
    }

    println!("Popped: {}\n", arr[*top - 1]);
    *top -= 1;
    arr[*top] = 0;
}

fn print(arr: [i32; 10]) {
    println!("{:?}\n", arr);
}

fn peek(arr: [i32; 10], top: usize) {
    if top == 0 {
        println!("Stack is empty...\n");
        return;
    }

    println!("Value at top: {}\n", arr[top - 1]);
}

fn main() {
    let mut stack: [i32; 10] = [0; 10];
    let mut top: usize = 0;

    loop {
        let operation = operation_inputs();
        
        match operation {
            1 => {
                let number = taking_inputs();
                push(&mut stack, &mut top, number);
            },
            2 => {
                pop(&mut stack, &mut top);
            },
            3 => {
                peek(stack, top);
            },
            4 => {
                print(stack);    
            },
            _ => {
                println!("Exiting...");
                break;
            }
        }
    }
}
