fn main() {
    let arr: [u8; 100] = [7, 66, 87, 37, 87, 23, 67, 21, 48, 88, 11, 98, 63, 72, 23, 53, 69, 10, 85, 26, 11, 61, 44, 33, 62, 91, 43, 42, 3, 17, 75, 52, 2, 93, 57, 45, 36, 93, 55, 9, 33, 88, 86, 24, 10, 47, 81, 61, 4, 85, 67, 65, 73, 60, 66, 42, 61, 15, 9, 100, 99, 98, 69, 84, 30, 19, 67, 80, 36, 58, 79, 93, 34, 65, 16, 49, 95, 66, 68, 62, 26, 92, 5, 43, 72, 35, 68, 96, 54, 31, 57, 93, 46, 84, 25, 2, 83, 6, 59, 14];
    let avg: u8 = average(arr);
    let (min, max) = minimum_maximum(arr);

    println!("Array: {arr:?}\n\nAverage: {avg}\nMaximum: {max}\nMinimum: {min}");
}

fn minimum_maximum(arr: [u8; 100]) -> (u8, u8) {
    let mut maximum: usize = 0;
    let mut minimum: usize = 10000;

    for i in arr {
        if usize::from(i) > maximum as usize {
            maximum = i as usize;
        } 
        if usize::from(i) < minimum as usize {
            minimum = i as usize;
        }
    }

    return (minimum as u8, maximum as u8);
}

fn average(arr: [u8; 100]) -> u8 {
    let mut sum: usize = 0;

    for i in arr {
        sum += i as usize;
    }

    return (sum / 100) as u8;
}
