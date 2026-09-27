fn main() {
    let x = 10;

    if x > 5 {
        println!("x is greater than 5");
    } else {
        println!("x is less than 5");
    }

    let condition = true;
    let number = if condition {5} else {3};
    println!("Since condition is {condition}, value of number is {number}");
}
