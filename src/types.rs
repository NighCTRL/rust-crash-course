/*
These are the primitive types
// numbers afters i & u are bits taken in memory
// u = unsigned, meaning it can't be a negative integer
Integers (u = unsigned): u8, i8, u16, i16, u32, i32, u64, i64, u128, i128
Floats: f32, f64
Characters (char)
Tuples
Arrays -> fixed length
vectors: growable arrays
*/

// Statically typed language = must know type of every variable at compile time.
// The compiler can sometime infer the type of the variable
pub fn run() {
    // type of i32
    let age = 30;
    // type of float64
    let days = 4.2;
    // add a type
    let y: i64 = 489033884893284;
    // find max size 
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);
    // boolean
    let is_active: bool = true;
    println!("state of is_active: {:?}", (is_active));
    // boolean from expression
    let is_greater: bool = 10 < 5;
    println!("state of is_greater: {:?}", (is_greater));
    // type of char
    let a1 = 'a';
    let emoji = '👌';
    let emoji_unicode = '\u{1F44C}';
    println!("char -> {:?}", (a1));
    println!("emoji -> {:?}", (emoji));
    println!("emoji_unicode -> {:?}", (emoji_unicode));
}
