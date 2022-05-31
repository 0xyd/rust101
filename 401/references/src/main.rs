fn main() {

    // Case 1.
    let s0 = String::from("Taiwan no 1.");
    let s0_len = calc_str_len(&s0);
    println!("{:?}", s0);
    println!("Length of s0: {:?}", s0_len);

    // Case 2.
    let mut s1 = String::from("Hello ");
    println!("s1 before world added {:?}", s1);
    add_world(&mut s1);
    println!("s1 after  world added {:?}", s1);

    // Case 3.
    let r1 = &mut s1;
    let r2 = &mut s1;

    // println!("r1 {:?}, r2 {:?}", r1, r2);

    {
        let r3 = &mut s1;
        add_world(r3);
        println!("In scope: {:?}", r3);
    }
    println!("After r3 called in scope, s1 :{:?}", s1);

    // Case 4.
    let mut s2 = String::from("Small ");
    add_world(&mut s2);
    println!("s2 :{:?}", s2);
    let r3 = &s2;
    let r4 = &s2;
    println!("r3: {:?} , r4: {:?}", r3, r4);

    // let r5 = &mut s2; // rustc complains 
    // add_world(&mut s2); // rustc complains

    {
        let r5 = &mut s2;
        add_world(r5);
        println!("r5: {:?}", r5);
    }
    println!("After r5 called in scope, s2 :{:?}", s2);

    
}

fn calc_str_len(s: &String) -> usize {
    s.len()
}

fn add_world(s: &mut String) {
    s.push_str("World!");
}