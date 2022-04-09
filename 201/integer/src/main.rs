fn main() {
    let a = 1_000;
    let b = 0x3E8;
    let c = 0o1750;
    let d = 0b11_1110_1000;

    if a == b && b == c && c == d {
        println!(
            "a:{}, b:{}, c: {} and d: {} are the same",
            a, b, c, d);    
    } else {
        println!("They are not the same!");
    }

    let e = b'A';

    if e == 65 {
        println!("A equals to {}", e);
    }
    
}
