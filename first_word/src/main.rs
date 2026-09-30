use std::io;

fn main() {
    println!("Enter a string:");
    let mut sentence = String::new();
    io::stdin()
        .read_line(&mut sentence)
        .expect("Error occurred while reading the input.");
    
    let word = first_word(sentence);
    println!("First word is {word}");
}

fn first_word(string: String) -> String {
    let mut word = String::new();

    for i in string.chars() {
        if i == ' ' {
            break;
        }
        word += &i.to_string();
    }

   return word; 
}
