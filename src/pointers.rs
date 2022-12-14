// Point to a ressource in memory
pub fn run() {
// primitive array
    let arr1 = [1,2,3];
    let arr2 = arr1;

    println!("Values: {:?}", (arr1, arr2));

    // the same things with vectors (Non primitive)
    // if you assign another variable to a piece of data, the first variable will no longer hold
    // that value, you will need to use a reference (&) to point to the ressource
    // You can't point to it if it is not a primitve value to create a reference
    let vec1 = vec![1,2,3];
    let vec2 = &vec1;

    println!("Values: {:?}", (&vec1, vec2));
}
