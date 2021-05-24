pub fn run(){
    greeting("Hello", "Gibbs");
    let getch = add(5, 6);
    println!("{}", getch);
}

fn greeting(greet: &str, name: &str){
    println!("{}, {}", greet, name);
}

fn add(num1: i32, num2: i32) -> i32{
    num1 + num2
}