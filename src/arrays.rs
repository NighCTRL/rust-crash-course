// Array -> Fixed list where elements are the same data types
pub fn run() {
    let mut numbers: [i32; 5] = [1,2,3,4,5];

    println!("{:?}", numbers);
    println!("Single value: {}", numbers[0]);
    // mutating an array
    numbers[2] = 30;
    println!("{:?}", numbers);
    // Amount of memory taken
    // Arrays are stack allocated
    // &numbers is to reference the numbers
    println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));
    let slice: &[i32] = &numbers[0..2];
    println!("{:?}", slice);
}
