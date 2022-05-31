fn main() {

    // Case 1: 
    let x = 1;
    let y = x;

    println!("x: {:?}, y: {:?}", x, y);
    println!("&x: {:p}, &y: {:p}", &x, &y);

    // Case 2
    let s1 = String::from("test");
    println!("s1: {:?}, &s1: {:p}", s1, &s1);

    let s2 = s1;
    println!("s2: {:?}, &s2: {:p}", s2, &s2);

    // Case 3
    let s3 = String::from("test");
    let s4 = s3.clone();
    println!("s3: {:?}, &s3: {:p}", s3, &s3);
    println!("s4: {:?}, &s4: {:p}", s4, &s4);

}
