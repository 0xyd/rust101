fn main() {

    let array = [1, 2, 3, 4, 5]; // Immutable array

    // Empty array is forbidden.
    // let empty_array = []; 
    let mut mutable_array = [1, 2, 3, 4, 5];

    let array2:[i32; 5];

    println!("Mutale array:");
    println!(
        "Before update: {} {} {} {} {}", 
        mutable_array[0],
        mutable_array[1],
        mutable_array[2],
        mutable_array[3],
        mutable_array[4]);

    mutable_array[0] *= 1000;
    mutable_array[1] *= 1000;
    mutable_array[2] *= 1000;
    mutable_array[3] *= 1000;
    mutable_array[4] *= 1000;

    println!(
        "After update: {} {} {} {} {}", 
        mutable_array[0],
        mutable_array[1],
        mutable_array[2],
        mutable_array[3],
        mutable_array[4]);

    // Access a non-initialized array is forbidden.
    // println!("Access non-initialized array:");
    // println!(
    //     "{} {} {} {} {}", 
    //     array2[0],
    //     array2[1],
    //     array2[2],
    //     array2[3],
    //     array2[4]);


}
