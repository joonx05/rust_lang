fn main() {
    let mut s = String::from("Hello");
    let s_ref = &mut s;
    println!("{&mut s}");
    s_ref.push_str(" world");
    }