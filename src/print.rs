pub fn run() {
    println!("Printing from the print.rs file");

    // formatting
    println!("This is basic {}", "formatting");

    // Positional arguments
    println!("{0} is playing {1} and {0} hates it", "Brandon", "Baseball");

    // named arguments
   println!("{arg_type} -> {name} is playing {sport} and {name} hates it", 
       arg_type = "named arguments",
        name = "Brandon", 
        sport = "Baseball"
    );

    //debuging
    // TODO: tuple
    println!("{:?}", (12, true, "Brandon"));

    // maths
    println!("10 + 10 = {}", 10 + 10);
}
