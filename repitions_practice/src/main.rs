fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("Value of result is {result}");

    counter = 0;

    'loop_one: loop {
        println!("Counter is {counter}");
        let mut counter_two = 0;

        loop {
            println!("Counter two is {counter_two}");
            counter_two += 1;

            if counter_two == 2 {
                break;
            }
            if counter == 2 {
                break 'loop_one;
            }
        }

        counter += 1;
    }

    counter = 10;

    while counter > 5 {
        println!("Counter is {counter}");
        counter -= 1;
    }

    let a = [10, 20, 30, 40, 50, 60];

    for element in a {
        println!("Element is {element}");
    }

    for number in 1..10 {
        println!("Forward counting {number}");
    }

    for number in (1..10).rev() {
        println!("Backward counting {number}");
    }
}
