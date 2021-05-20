pub fn run(){
    let person: (&str, &str, i8) = ("gibbs", "sg", 20);

    println!("{}, {}, {}", person.0, person.1, person.2);
}