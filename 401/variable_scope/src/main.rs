

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

    // Case 3.
    let mut c = 100;
    add_1(c);

    let d = 1;
    add_1(d);

    // Case 4.
    let mut e = 10;
    e = radd_1(e);
    println!("{:?}", e);    

}

fn add_1(mut x: i32){
    x = x+1;
    println!("{:?}", x);
}

fn radd_1(mut x: i32) -> i32 {
    x += 1;
    return x;
}