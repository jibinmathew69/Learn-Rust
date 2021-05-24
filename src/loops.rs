pub fn run() {
    let mut count = 0;

    loop {
        count += 1;
        println!("{}", count);

        if count == 10 {
            break;
        }
    }

    while count <= 100 {
        if count % 15 == 0 {
            println!("fizzbuzz");
        }
        count += 1;
    }

    for x in 0..10{
        println!("{}", x);
    }
}
