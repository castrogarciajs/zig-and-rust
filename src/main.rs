fn main() {
    println!("Hello, world!");
    let hello = great();

    println!("{hello}")
}


fn great() -> String {
    String::from("Hello world")
}