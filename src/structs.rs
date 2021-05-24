struct Color{
    red: u8,
    green: u8,
    blue: u8
}

struct Person{
    first_name: String,
    last_name: String
}

impl Person{
    fn new(first: &str, last: &str) -> Person{
        Person{
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    fn full_name(&self) -> String{
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_lastname(&mut self, last: &str){
         self.last_name = last.to_string();
    }
}


pub fn run(){

    let mut c = Color{
        red: 255, 
        green: 100,
        blue: 50
    };

    println!("{}, {}, {}", c.red, c.blue, c.green);

    let mut p = Person::new("Gibbs", "Matt");

    println!("{} {}", p.first_name, p.last_name);

    println!("{}", p.full_name());

    p.set_lastname("John");

    println!("{}", p.full_name());

}