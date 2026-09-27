// this prints all the counting numbers from 10 to 99 while indicating whether they are multiples of 2, 3, or 5

fn main() {
    let mut multiple_string;
    let mut counter = 10;

    while counter < 100 {
        multiple_string = String::new();
        multiple_string += "- ";

        if counter % 2 == 0 {
            multiple_string += "multiple of 2\t"; 
        } 

        if counter % 3 == 0 {
            multiple_string += "multiple of 3\t";
        } 

        if counter % 5 == 0 {
            multiple_string += "multiple of 5\t";
        }
        
        println!("{counter}\t{multiple_string}");
        counter += 1;
    }

}
