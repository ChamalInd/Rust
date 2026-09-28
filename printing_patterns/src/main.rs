fn main() {
    let mut pattern = String::new();

    for _ in 0..11 {
        pattern += "*";
    }
    println!("{pattern}\n");

    pattern = String::new();

    for _ in 0..5 {
        pattern += "*\n";
    }
    println!("{pattern}\n");

    pattern = String::new();

    for _ in 0..5 {
        for _ in 0..8 {
            pattern += "*";
        }
        pattern += "\n";
    }
    println!("{pattern}\n");

    pattern = String::new();

    for i in 1..6 {
        for _ in 0..i { 
            pattern += "*";
        }
        pattern += "\n";
    }
    println!("{pattern}\n");

    pattern = String::new();

    for i in 1..10 {
        let upper = if i < 5 {i} else {5 - (i % 5)}; 

        for _ in 0..upper {
            pattern += "*";
        }
        pattern += "\n";
    }
    println!("{pattern}\n");

    pattern = String::new();

    for i in 1..10 {
        let upper = if i < 5 {5 - i} else {i % 5};
        for _ in 0..upper {
            pattern += " ";
        }

        let chr_upper = (if i < 5 {i} else {5 - (i % 5)}) * 2 - 1;
        for _ in 0..chr_upper {
            pattern += "*";
        }
        pattern += "\n";
    }
    println!("{pattern}\n");

    pattern = String::new();

    for i in 1..6 {
        for _ in 0..(i - 1) {
            pattern += " ";
        }
        for _ in 0..8 {
            pattern += "*";
        }
        pattern += "\n";
    }
    println!("{pattern}\n");
}
