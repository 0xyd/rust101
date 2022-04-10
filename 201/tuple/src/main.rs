fn main() {

    let empty_tup = (); // empty tuple is acceptable

    let tup:(char, char, char, char, char, u8) = (
        '台', '灣', 'N', 'o', '.', 1);
    println!(
        "{}{}{}{}{}{}", 
        tup.0, tup.1, tup.2, 
        tup.3, tup.4, tup.5);

}
