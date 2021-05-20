use std::mem;

pub fn run(){
    let mut numbers: [i32; 5] = [1, 2, 2, 3, 3];

    println!("{:?}", numbers);
    println!("{}", numbers[2]);

    numbers[2] = 3;

    println!("{:?}", numbers);
    println!("{}", numbers.len());

    println!("{}", mem::size_of_val(&numbers));

    let slice: &[i32] = &numbers[1..3];
    println!("{:?}", slice);
}