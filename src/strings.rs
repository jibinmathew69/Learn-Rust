pub fn run(){
    let hello = "Hello";
    let mut hello = String::from("Hello");

    println!("{}", hello);
    println!("{}", hello.len());

    hello.push('W');
    println!("{}", hello);

    hello.push_str("oo");
    println!("{}", hello);

    println!("{}", hello.capacity());

    println!("{}", hello.is_empty());
}