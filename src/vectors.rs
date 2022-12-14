// vectors are resizeable arrays
pub fn run(){
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];
    println!("{:?}", numbers);
    println!("Single value: {}", numbers[0]);
    //adding to the vector
    numbers.push(6);
    numbers.push(7);
    numbers.pop();
    // Loop throught the values of a vector
    for n in numbers.iter() {
        println!("n: {}", n)
    }

    // loop and mutate
    for n in numbers.iter_mut() {
        *n *= 2;
    }

    println!("after mutation in loop: {:?}", numbers)
}
