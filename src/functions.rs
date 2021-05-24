pub fn run(){
    greeting("Hello", "Gibbs");
    let getch = add(5, 6);
    println!("{}", getch);

    let add_nums = |n1: i32, n2: i32| n1+n2;
    println!("{}", add_nums(3, 4)); 
}

fn greeting(greet: &str, name: &str){
    println!("{}, {}", greet, name);
}

fn add(num1: i32, num2: i32) -> i32{
    num1 + num2
}