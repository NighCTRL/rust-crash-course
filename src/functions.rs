// Store block of code of reuse
pub fn run() {
    greeting("Hi", "Mark");
    println!("addition is {}", add(5, 10));
    // same as
    // sum = add(5, 10)
    // println!("addition is {}", sum)

    // Closures
    let n3 = 30;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("Sum of closure: {}", add_nums(7, 45));
}

fn greeting(greet: &str, name: &str){
    println!("{} {}", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    // no semicolon at the end to return that value
    n1 + n2
}
