pub fn run() {
    let name = "brandon";
    let age = "43";

    println!("my name is {} and I am {}", name, age);

    // if using keyword const you have to define a task
    const ID: i32 = 001;
    println!("id = {}", ID);

    // assign multiple variables at once
    let (my_name, my_age) = ("Brandon", 44);
    println!("{} is {}", my_name, my_age)
}
