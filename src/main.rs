use std::io;

fn main() {
    println!("Hello from rusty-weather");

    println!("Please input your local city");

    let mut city = String::new();

    io::stdin()
        .read_line(&mut city)
        .expect("Fasiled to read line");

    println!("The city you entered is: {city}");
}
