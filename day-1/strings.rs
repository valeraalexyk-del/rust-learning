fn main() {
    let name: &str = "Rust";
    let mut message: String = String::from("Привет, ");

    message.push_str(name);

    println!("{}", message);
}
