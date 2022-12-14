// Two types of string
// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data-structure -- used when you need to modify the string
pub fn run(){
    // Primitiv
    let greet = "Hello "; 
    // Growable
    let mut grow_greet = String::from(greet);
    println!("{} from strings.rs", grow_greet);
    // print the lenght of the string
    println!("Length: {}", grow_greet.len());
    // change the string
    // must have String::from("hello ") to be able to use these methods
    // one character
    grow_greet.push('w');
    // push a string
    grow_greet.push_str("orld !");
    println!("{} from strings.rs", grow_greet);
    println!("Length: {}", grow_greet.len());
    // There is a lot of methods for strings
    //capacity in bytes
    println!("Capacity: {}", grow_greet.capacity());
    // is empty
    println!("Is empty? {}", grow_greet.is_empty());
    // does it contain [pattern]
    println!("Contains world? {}", grow_greet.contains("world"));
    // replace
    println!("Replace: {}", grow_greet.replace("world", "Brandon"));
    // looping through string by white space
    for word in grow_greet.split_whitespace() {
        println!("{}", word);
    }
    // Create a string with a certain capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    println!("{}", s);

    // Assertion testing
    // only stops when false
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());
}
