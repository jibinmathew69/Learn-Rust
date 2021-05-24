struct Color{
    red: u8,
    green: u8,
    blue: u8
}

pub fn run(){

    let mut c = Color{
        red: 255, 
        green: 100,
        blue: 50
    };

    println!("{}, {}, {}", c.red, c.blue, c.green);

}