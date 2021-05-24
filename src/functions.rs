pub fn run(){
    greeting("Hello", "Gibbs");
}

fn greeting(greet: &str, name: &str){
    println!("{}, {}", greet, name);
}