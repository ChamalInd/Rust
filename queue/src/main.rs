use std::io;

fn input_operation() -> i32 {
    println!("1: enqueue\n2: dequeue\n3: peek\n4: print\n");
    println!("Enter an operation:");
        
    let mut oper = String::new();
    io::stdin()
        .read_line(&mut oper)
        .expect("Error occurred while reading the operation");
    let oper: i32 = match oper.trim().parse() {
        Ok(num) => num,
        Err(_) => -1 
    };

    return oper;
}

fn input_number() -> i32 {
    loop {
        println!("Enter a number:");
        let mut num = String::new();
        io::stdin()
            .read_line(&mut num)
            .expect("Error occurred while reading the number");
        let num: i32 = match num.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter a correct number\n");
                continue;
            }
        };

        return num;
    }
}

fn enqueue(arr: &mut [i32; 10], tail: &mut usize, num: i32) {
    if *tail == 10 {
        println!("Queue is full...\n");
        return;
    }

    arr[*tail] = num;
    *tail += 1;

    println!("Enqueued {num} to {}\n", *tail);
}

fn dequeue(arr: &mut [i32; 10], tail: &mut usize) {
    if arr[0] == 0 {
        println!("Queue is empty...\n");
        return;
    }
    
    if *tail > 0 {
        println!("Dequeued: {}\n", arr[0]);
        
        for i in 0..9 {
            arr[i] = arr[i + 1];
        }
        *tail -= 1;
        arr[*tail] = 0;
    }
}

fn print(arr: [i32; 10]) {
    println!("{:?}\n", arr);
}

fn peek(arr: [i32; 10]) {
    if arr[0] == 0 {
        println!("Queue is empty...\n");
        return;
    }

    println!("Value at head: {}\n", arr[0]);
}

fn main() {
    let mut queue: [i32; 10] = [0; 10];
    let mut tail: usize = 0;

    loop {
        let oper = input_operation();
        
        match oper {
            1 => {
                let num = input_number();
                enqueue(&mut queue, &mut tail, num);
            },
            2 => {
                dequeue(&mut queue, &mut tail);
            },
            3 => {
                peek(queue);
            }, 
            4 => {
                print(queue);
            },
            _ => {
                println!("Exiting...");
                break;
            }
        }
    }
}
