// Used to check condition and act on the result
pub fn run() {
    let age: u8 = 19;
    let check_id: bool = true;
    let knows_person_of_age = true;
    if age >= 18 || knows_person_of_age {
        println!("Good you can drink");
    } else if age < 18 && check_id {
        println!("You can't drink");
    } else {
        println!("Can i see your ID");
    }

    // Shorthand if
    let is_of_age = if age >= 18 { true } else { false};
    println!("Is of age? {}", is_of_age)
}
