fn main() {
    let s1 = String::from("hola");
    let mut v = Vec::new();
    v.push(s1);
    let s2: &String = &v[0];
    println!("{}", s2);
}


/*fn drip_drop() -> String {
    let s: String = String::from("hello world!");
    return s;
}
fn main () {
    let s = drip_drop();
    println!("{}", s);
}
*/
/*fn main() {
    let mut s = String::from("hola");
    let ref1 = &s;
    let ref2 = &ref1;
    let ref3 = &ref2;
    println!("{}", ref3.to_uppercase());
    s = String::from("chau");
}*/