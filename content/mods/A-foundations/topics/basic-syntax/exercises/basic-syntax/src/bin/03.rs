fn main() {
    let input = [23, 82, 16, 45, 21, 94, 12, 34];
    let mut largest = input[0];
    let mut smallest = input[0];
    for i in input.iter() {
        if *i > largest {
            largest = *i;
        }
        if *i < smallest {
            smallest = *i;
        }
    }
    

    println!("{largest} is largest and {smallest} is smallest");
}
