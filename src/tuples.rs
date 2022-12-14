// tuples group together values of different types
// Max 12 elements
pub fn run() {
    let person: (&str, &str, i8) = ("Brandon", "Chicago", 37);
    println!("{} is from {} and aged {}", person.0, person.1, person.2)
}
