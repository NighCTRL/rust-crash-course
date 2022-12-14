// loop = iterate until condition is met
pub fn run() {
    let mut count = 0;
    // infinite loop
    loop {
        count += 1;
        println!("Count: {}", count);
        
        if count == 10 {
            break;
        }
    }

    // while loop (FizzBuzz)
    // loop 1 -> 100 
    // if number divisible by 3 = print fizz
    // if number divisible by 5 = print buzz
    // if both divisible by 3 and 5 = print fizzBuzz
    while count <=100 {
        // == 0 because % (the modulus) gives us the remainder
        if count % 15 == 0 {
            println!("fizzBuzz");
        } else if count % 3 == 0 {
            println!("fizz");
        } else if count % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", count);
        }
        count += 1
    }

    // For range loop
    for n in 0..100 {
        if n % 15 == 0 {
            println!("fizzBuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}
