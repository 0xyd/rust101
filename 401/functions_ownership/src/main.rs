fn main() {
    let a = 10; 
    let b = String::from("TAIWAN NO 1.");

    makes_copy(a);
    takes_ownership(b);

    println!("a {:?} is still workable in main.", a);
    // println!("{:?}", b); rust complains here since ownership is taken

    let c = String::from("1 + 2 = ");
    manipulates_string(c);

    let d0 = String::from("1 + 2 = ");
    let d1 = rmanipulates_string(d0);
    println!("{:?}", d1);

}

fn makes_copy(x: i32) {
    println!("{:?} is coppied.", x);
}

fn takes_ownership(s: String) {
    println!("{:?} 's ownership is taken", s);
}

fn manipulates_string(mut s: String) {
    s.push('3');
    println!("{:?}", s);
}

fn rmanipulates_string(mut s: String) -> String {
    s.push('3');
    // return s;
    s
}

