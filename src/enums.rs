enum Movement{
    Up, 
    Down, 
    Left, 
    Right
}

fn move_dir(m: Movement){
    
    match m {
        Movement::Up => println!("Up"),
        Movement::Down => println!("Down"),
        Movement::Left => println!("Left"),
        Movement::Right => println!("Right")
    }
    
}

pub fn run(){
    let obj = Movement::Up;
    move_dir(obj);
}