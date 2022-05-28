

fn main() {

    // Case 1.
    {
        let a = "a";
        println!("{:?}", a);
    }
    // println!("{:?}", a); // Bug! a not found in this scope.

    // Case 2.
    let b = "b";
    {
        println!("{:?}", b);
    }

}
