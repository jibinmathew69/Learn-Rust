pub fn run(){
    println!("Hello printrs");
    println!("num: {}", 1);
    println!("{0} is {1} but {0} is not {2}", "x", 0, 1);
    println!("{name} is a good {job}", name="Gibbs", job="engineer");
    println!("{:b} {:o} {:x}", 10, 10, 10);
}